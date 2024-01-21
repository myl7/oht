// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

#pragma once

#include <cstdint>

namespace oht {

constexpr uint32_t kTagDummy = 1 << 0;

namespace tag {

constexpr uint32_t kTagFiller = 1 << 1;
constexpr uint32_t kTagExcess = 1 << 2;

inline uint32_t init() {
  return 0;
}

inline uint32_t set(uint32_t x, uint32_t tag) {
  return x | tag;
}

inline uint32_t unset(uint32_t x, uint32_t tag) {
  return x & ~tag;
}

inline bool check(uint32_t x, uint32_t tag) {
  return (x & tag) > 0;
}

inline uint8_t get(uint32_t x, uint32_t tag) {
  return check(x, tag) ? 1 : 0;
}

}  // namespace tag

}  // namespace oht
