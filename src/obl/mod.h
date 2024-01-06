// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

#pragma once

#include "rust/cxx.h"
#include "oht/include/obl/ffi.rs.h"
#include <cstddef>
#include <cstdint>

void osort(rust::Slice<Elem> vec, rust::Fn<bool(const Elem &, const Elem &)> cmp, size_t jobs);
bool olt(uint32_t a, uint32_t b);
bool ogt(uint32_t a, uint32_t b);
bool oeq(uint32_t a, uint32_t b);
uint32_t ochoose_u32(bool pred, uint32_t a, uint32_t b);
bool ochoose_bool(bool pred, bool a, bool b);
