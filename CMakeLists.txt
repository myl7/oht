cmake_minimum_required(VERSION 3.16)
project(oht LANGUAGES CXX)

add_library(oht INTERFACE)
target_include_directories(oht INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}/include")
set_property(TARGET oht PROPERTY CXX_STANDARD 17)

include(CTest)
if(BUILD_TESTING)
  add_subdirectory(tests)
endif()
