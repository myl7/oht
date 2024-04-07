# SPDX-License-Identifier: Apache-2.0

# For Ubuntu 20.04 Focal so far, which is the latest one officially supported by openenclave so far
cmake_minimum_required(VERSION 3.16)

# The original code is at the URL https://github.com/openenclave/openenclave,
# in the path samples/helloworld/CMakeLists.txt,
# owned by Open Enclave SDK contributors, and licensed under the MIT License.
# The current code is redistributed under the Apache License, Version 2.0.
# The original file header is:
#
# Copyright (c) Open Enclave SDK contributors.
# Licensed under the MIT License.
# (End of the original file header)
#
# The original LICENSE file is embedded below:
#
# MIT License
#
# Copyright (c) Open Enclave SDK contributors.
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE
# (End of the original LICENSE file)
#
# The current code is used below as some code blocks.
# The code blocks are marked by ## oecode.

## oecode
if (LVI_MITIGATION MATCHES ControlFlow)
  # Configure the cmake to use customized compilation toolchain.
  # This package has to be added before `project()`.
  find_package(OpenEnclave-LVI-Mitigation CONFIG REQUIRED)
else ()
  # Setting the cmake compiler when LVI mitigation is not enabled. If the CC
  # environment variable has been specified or the if CMAKE_C_COMPILER cmake
  # variable has been passed to cmake, use the C compiler that has been specified.
  # Otherwise, prefer clang. Same for C++ compiler. This must be done before
  # the `project` command.
  if (UNIX)
    if (NOT DEFINED ENV{CC} AND NOT DEFINED CMAKE_C_COMPILER)
      find_program(CMAKE_C_COMPILER clang-11 clang-10 clang)
    endif ()
    if (NOT DEFINED ENV{CXX} AND NOT DEFINED CMAKE_CXX_COMPILER)
      find_program(CMAKE_CXX_COMPILER clang++-11 clang++-10 clang++)
    endif ()
  endif ()
endif ()

project(oht-enclave LANGUAGES CXX C)
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

## oecode
set(OE_CRYPTO_LIB openssl_3 CACHE STRING "Crypto library used by enclaves")

find_package(OpenEnclave REQUIRED)
add_subdirectory(lib/oht)

add_executable(
  oht-enclave
  api/oht_t.c
  src/api.cpp
  src/oht_crypto_impl.cpp
  src/oht_snoopy_impl.cpp
)
target_compile_definitions(oht-enclave PUBLIC OE_API_VERSION=2)
target_include_directories(oht-enclave PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})
target_include_directories(oht-enclave PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_compile_options(oht-enclave PUBLIC -mavx2)
target_compile_definitions(
  oht-enclave PUBLIC
  # OHT_SNOOPY_AVX2=1
  OHT_OBLSORT_SINGLETHREAD=1
)

## oecode
if (LVI_MITIGATION MATCHES ControlFlow)
  # Helper to enable compiler options for LVI mitigation.
  apply_lvi_mitigation(enclave)
  # Link against LVI-mitigated libraries.
  target_link_libraries(
    oht-enclave
    openenclave::oeenclave-lvi-cfg
    openenclave::oecrypto${OE_CRYPTO_LIB}-lvi-cfg
    openenclave::oelibcxx-lvi-cfg
  )
else ()
  target_link_libraries(
    oht-enclave PRIVATE
    openenclave::oeenclave
    openenclave::oecrypto${OE_CRYPTO_LIB}
    openenclave::oelibcxx
  )
endif ()
target_link_libraries(oht-enclave PRIVATE oht)
