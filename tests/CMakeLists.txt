include(CTest)
set(CMAKE_CXX_STANDARD 17)

find_package(OpenSSL REQUIRED)

target_compile_options(oht INTERFACE -mavx2)
target_compile_definitions(
  oht INTERFACE
  OHT_SNOOPY_AVX2=1
  OHT_OBLSORT_SINGLETHREAD=1
)

add_executable(
  test_oht_build
  test_oht_build.cpp
  oht_crypto_impl.cpp
  snoopy_par_obl_primitives_impl.cpp
)
target_compile_options(test_oht_build PRIVATE -mavx2)
target_link_libraries(
  test_oht_build PRIVATE
  oht
  OpenSSL::Crypto
)
add_test(NAME test_oht_build COMMAND test_oht_build)
