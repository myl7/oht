# For Ubuntu 20.04 Focal so far, which is the latest one officially supported by openenclave so far
cmake_minimum_required(VERSION 3.16)
project(
  oht
  VERSION 0.1.1
  DESCRIPTION "Oblivious 2-tier hash table C++ header-only library"
  HOMEPAGE_URL "https://github.com/myl7/oht"
  LANGUAGES CXX
)

add_library(oht INTERFACE)
target_include_directories(oht INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}/include")
target_compile_features(oht INTERFACE cxx_std_17)

include(CTest)
if(BUILD_TESTING)
  add_subdirectory(tests)
endif()
