// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

#include "oht/src/obl/mod.h"
#include <cassert>
#include <thread>
#include "par_obl_primitives.h"
#include "oeq_arr.h"

void osort(rust::Slice<Elem> vec, rust::Fn<bool(const Elem &, const Elem &)> cmp, size_t jobs) {
  assert((void("jobs can not be 0"), jobs != 0));
  if (jobs == 1) {
    ObliviousSortParallel(vec.begin(), vec.end(), cmp, jobs, 0);
  } else {
    std::thread workers[jobs - 1];
    for (size_t i = 1; i < jobs; i++) {
      workers[i - 1] =
        std::thread(ObliviousSortParallel<rust::Slice<Elem>::iterator, rust::Fn<bool(const Elem &, const Elem &)>>,
          vec.begin(), vec.end(), cmp, jobs, i);
    }
    ObliviousSortParallel(vec.begin(), vec.end(), cmp, jobs, 0);
    for (size_t i = 1; i < jobs; i++) {
      workers[i - 1].join();
    }
  }
}

bool olt(uint32_t a, uint32_t b) {
  return ObliviousLess(a, b);
}

bool ogt(uint32_t a, uint32_t b) {
  return ObliviousGreater(a, b);
}

bool oeq(uint32_t a, uint32_t b) {
  return ObliviousEqual(a, b);
}

bool oeq_key(std::array<uint8_t, KEY_SIZE> a, std::array<uint8_t, KEY_SIZE> b) {
  return oeq_arr(a.data(), b.data(), KEY_SIZE);
}

uint32_t ochoose_u32(bool pred, uint32_t a, uint32_t b) {
  return ObliviousChoose(pred, a, b);
}

bool ochoose_bool(bool pred, bool a, bool b) {
  return ObliviousChoose(pred, a, b);
}

std::array<uint8_t, VAL_SIZE> ochoose_val(bool pred, std::array<uint8_t, VAL_SIZE> a, std::array<uint8_t, VAL_SIZE> b) {
  return ObliviousChoose(pred, a, b);
}
