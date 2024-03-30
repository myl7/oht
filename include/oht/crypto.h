// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

#pragma once

#include <cstddef>
#include <cstdint>
#include "types.h"

namespace oht::decl {

using oht::PrfKey;

uint32_t PrfInt(const PrfKey &key, const uint8_t *data, size_t data_size, uint32_t max_plus1);

}  // namespace oht::decl
