// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

#pragma once

#include "rust/cxx.h"
#include "oht/include/obl/ffi.rs.h"

#ifndef KEY_SIZE
#error "KEY_SIZE must be defined"
#endif
#ifndef VAL_SIZE
#error "VAL_SIZE must be defined"
#endif

void osort(rust::Slice<Elem> vec, rust::Fn<bool(const Elem &, const Elem &)> cmp, size_t jobs);
bool olt(uint32_t a, uint32_t b);
bool ogt(uint32_t a, uint32_t b);
bool oeq(uint32_t a, uint32_t b);
bool oeq_key(std::array<uint8_t, KEY_SIZE> a, std::array<uint8_t, KEY_SIZE> b);
uint32_t ochoose_u32(bool pred, uint32_t a, uint32_t b);
bool ochoose_bool(bool pred, bool a, bool b);
std::array<uint8_t, VAL_SIZE> ochoose_val(bool pred, std::array<uint8_t, VAL_SIZE> a, std::array<uint8_t, VAL_SIZE> b);
