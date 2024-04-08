#include <algorithm>
#include <random>
#include <string>
#include <cstddef>
#include <cstdint>
#include <cassert>
extern "C" {
#include "api/oht_u.h"
#include <openenclave/host.h>
}

constexpr size_t kVecLen = 1000;
static std::string kEnclavePath = "build/enclave/oht-enclave.signed";

int main() {
  std::vector<uint32_t> vec;
  for (uint32_t i = 0; i < kVecLen; i++) {
    vec.push_back(i);
  }

  std::mt19937 gen;
  gen.seed(114514);

  std::shuffle(vec.begin(), vec.end(), gen);

  oe_enclave_t *enclave;
  int res;
  res = oe_create_oht_enclave(
    kEnclavePath.c_str(), OE_ENCLAVE_TYPE_AUTO, OE_ENCLAVE_FLAG_DEBUG | OE_ENCLAVE_FLAG_SIMULATE, nullptr, 0, &enclave);
  assert((void("oe_create_oht_enclave failed"), res == 0));

  // TODO: Impl
  res = oht_build(enclave, nullptr, 0);
  assert((void("oht_build failed"), res == 0));

  size_t pos;
  res = oht_lookup(enclave, &pos, nullptr, 0);
  assert((void("oht_lookup failed"), res == 0));

  return 0;
}
