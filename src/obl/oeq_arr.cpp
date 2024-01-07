// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

#include "oeq_arr.h"
#include "par_obl_primitives.h"

// #ifdef USE_AVX2
// #include <intrinsics/immintrin.h>
// #endif

// Better to have a SIMD oblivious eq but it is not provided by `par_obl_primitives.h`
bool oeq_arr(const uint8_t *a, const uint8_t *b, size_t size) {
  size_t i = 0;
  bool res = true;
  while (i < size) {
    if (i + 8 <= size) {
      // These array accesses are oblivious because the indices are deterministic.
      // Same for the following ones.
      res &= ObliviousEqual(*reinterpret_cast<const uint64_t *>(a + i), *reinterpret_cast<const uint64_t *>(b + i));
      i += 8;
    } else if (i + 4 <= size) {
      res &= ObliviousEqual(*reinterpret_cast<const uint32_t *>(a + i), *reinterpret_cast<const uint32_t *>(b + i));
      i += 4;
    } else if (i + 1 <= size) {
      res &= ObliviousEqual(*(a + i), *(b + i));
      i += 1;
    }
  }
  return res;
}
