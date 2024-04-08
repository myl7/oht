add_executable(
  oht-host
  api/oht_u.c
  src/main.cpp
)
target_compile_definitions(oht-host PUBLIC OE_API_VERSION=2)
target_include_directories(oht-host PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(
  oht-host PRIVATE
  openenclave::oehost
  # Force to be OpenSSL 1.1
  /usr/lib/libcrypto.so.1.1
)
