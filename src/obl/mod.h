// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

#pragma once

#include "rust/cxx.h"
#include "oht/include/obl/ffi.rs.h"
#include <cstddef>

void osort(rust::Slice<Elem> vec, rust::Fn<bool(const Elem &, const Elem &)> cmp, size_t jobs);
