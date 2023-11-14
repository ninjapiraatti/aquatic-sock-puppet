# Distributed under the OSI-approved BSD 3-Clause License.  See accompanying
# file Copyright.txt or https://cmake.org/licensing for details.

cmake_minimum_required(VERSION 3.5)

file(MAKE_DIRECTORY
  "/Users/ninjapiraatti/esp/esp-idf/components/bootloader/subproject"
  "/Users/ninjapiraatti/Projektit/aquatic-sock-puppet/build/bootloader"
  "/Users/ninjapiraatti/Projektit/aquatic-sock-puppet/build/bootloader-prefix"
  "/Users/ninjapiraatti/Projektit/aquatic-sock-puppet/build/bootloader-prefix/tmp"
  "/Users/ninjapiraatti/Projektit/aquatic-sock-puppet/build/bootloader-prefix/src/bootloader-stamp"
  "/Users/ninjapiraatti/Projektit/aquatic-sock-puppet/build/bootloader-prefix/src"
  "/Users/ninjapiraatti/Projektit/aquatic-sock-puppet/build/bootloader-prefix/src/bootloader-stamp"
)

set(configSubDirs )
foreach(subDir IN LISTS configSubDirs)
    file(MAKE_DIRECTORY "/Users/ninjapiraatti/Projektit/aquatic-sock-puppet/build/bootloader-prefix/src/bootloader-stamp/${subDir}")
endforeach()
if(cfgdir)
  file(MAKE_DIRECTORY "/Users/ninjapiraatti/Projektit/aquatic-sock-puppet/build/bootloader-prefix/src/bootloader-stamp${cfgdir}") # cfgdir has leading slash
endif()
