// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

#include "oht/src/obl/mod.h"
#include "par_obl_primitives.h"

void osort(rust::Slice<Elem> vec, rust::Fn<bool(const Elem &, const Elem &)> cmp, size_t jobs) {
  ObliviousSortParallel(vec.begin(), vec.end(), cmp, jobs, 0);
}
