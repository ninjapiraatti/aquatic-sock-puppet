#![no_std]
#![no_main]

extern crate alloc;
use core::cell::RefCell;
use critical_section::Mutex;
use core::mem::MaybeUninit;
use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl,
    gpio::{
        Event,
        Gpio4,
        Gpio6,
        Input,
        Output,
        PullDown,
        IO,
        PushPull
    },
    peripherals::{self, Peripherals},
    interrupt,
    prelude::*,
    Delay
};

use esp_wifi::{initialize, EspWifiInitFor};

use hal::{systimer::SystemTimer, Rng};
#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();
static BUTTON: Mutex<RefCell<Option<Gpio6<Input<PullDown>>>>> = Mutex::new(RefCell::new(None));
static LED: Mutex<RefCell<Option<Gpio4<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}
#[entry]
fn main() -> ! {
    init_heap();
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    // setup logger
    // To change the log_level change the env section in .cargo/config.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Hello world!");

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut rotary_input = io.pins.gpio6.into_pull_down_input();
    let mut led = io.pins.gpio4.into_push_pull_output();
    led.set_high().unwrap();
    rotary_input.listen(Event::FallingEdge);

    critical_section::with(|cs| BUTTON.borrow_ref_mut(cs).replace(rotary_input));
    critical_section::with(|cs| LED.borrow_ref_mut(cs).replace(led));

    interrupt::enable(peripherals::Interrupt::GPIO, interrupt::Priority::Priority3).unwrap();

    let timer = SystemTimer::new(peripherals.SYSTIMER).alarm0;
    let _init = initialize(
        EspWifiInitFor::Wifi,
        timer,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    )
    .unwrap();
    loop {
        delay.delay_ms(1500u32);
    }
}

#[interrupt]
fn GPIO() {
    critical_section::with(|cs| {
        println!("GPIO interrupt");
        BUTTON
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();
        if let Some(led) = LED.borrow_ref_mut(cs).as_mut() {
            if led.is_set_high().unwrap() {
                led.set_low().unwrap();
            } else {
                led.set_high().unwrap();
            }
        }
    });
}