// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

#pragma once

#include "snoopy/obl_primitives.h"
#include "oblsort_dispatch.h"
#include "obleq_arr.h"

namespace oht::obl {

using decl::OblSort;
using obl::OblEqArr;

template <typename T>
inline bool OblLt(const T &a, const T &b) {
  return snoopy::ObliviousLess(a, b);
}

template <typename T>
inline bool OblLe(const T &a, const T &b) {
  return snoopy::ObliviousLessOrEqual(a, b);
}

template <typename T>
inline bool OblGt(const T &a, const T &b) {
  return snoopy::ObliviousGreater(a, b);
}

template <typename T>
inline bool OblGe(const T &a, const T &b) {
  return snoopy::ObliviousGreaterOrEqual(a, b);
}

template <typename T, typename std::enable_if<std::is_scalar<T>::value, int>::type = 0>
inline bool OblEq(const T &a, const T &b) {
  return snoopy::ObliviousEqual(a, b);
}

template <typename T, typename std::enable_if<std::is_standard_layout<T>::value, int>::type = 0>
inline void OblAssign(bool pred, const T &t_val, const T &f_val, T *out) {
  snoopy::ObliviousAssign(pred, t_val, f_val, out);
}

template <typename T>
inline T OblChoose(bool pred, const T &t_val, const T &f_val) {
  return snoopy::ObliviousChoose(pred, t_val, f_val);
}

template <typename Iter>
inline void OblMerge(Iter begin, Iter end) {
  return snoopy::ObliviousMerge(begin, end);
}

template <typename Iter, typename Cmp>
inline void OblMerge(Iter begin, Iter end, Cmp cmp) {
  return snoopy::ObliviousMerge(begin, end, cmp);
}

template <typename Iter>
inline void OblSortRaw(Iter begin, Iter end) {
  return snoopy::ObliviousSort(begin, end);
}

template <typename Iter, typename Cmp>
inline void OblSortRaw(Iter begin, Iter end, Cmp cmp) {
  return snoopy::ObliviousSort(begin, end, cmp);
}

template <typename Iter>
inline void OblCompact(Iter begin, Iter end, uint8_t *tags) {
  return snoopy::ObliviousCompact(begin, end, tags);
}

template <typename T>
inline T OblArrAccess(const T *arr, size_t i, size_t arr_len) {
  return snoopy::ObliviousArrayAccess(arr, i, arr_len);
}

inline void OblArrAccessBytes(void *dst, const void *arr, size_t size, size_t i, size_t arr_len) {
  return snoopy::ObliviousArrayAccessBytes(dst, arr, size, i, arr_len);
}

template <typename T>
inline T OblArrAccessSimd(const T *arr, size_t i, size_t arr_len) {
  return snoopy::ObliviousArrayAccessSimd(arr, i, arr_len);
}

template <typename T>
inline void OblArrAssign(T *arr, size_t i, size_t arr_len, const T &val) {
  return snoopy::ObliviousArrayAssign(arr, i, arr_len, val);
}

inline void OblArrAssignBytes(void *arr, const void *src, size_t size, size_t i, size_t arr_len) {
  return snoopy::ObliviousArrayAssignBytes(arr, src, size, i, arr_len);
}

}  // namespace oht::obl
