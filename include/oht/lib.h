// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

#pragma once

#include <vector>
#include <array>
#include <cstdint>
#include <cstring>
#include "tag.h"
#include "crypto.h"
#include "obl/mod.h"
#include "types.h"

// Decl
namespace oht {

template <size_t Ksize, size_t Vsize>
struct Elem {
  uint32_t tag = 0;
  uint32_t bin_id;
  uint8_t key[Ksize]{};
  uint8_t val[Vsize]{};
};

template <size_t Ksize, size_t Vsize>
class Oht {
 public:
  std::vector<Elem<Ksize, Vsize>> bins1_{};
  std::vector<Elem<Ksize, Vsize>> bins2_{};
  size_t kB_;
  size_t kZ_;

  Oht(size_t b, size_t z) : kB_(b), kZ_(z) {}

  void clear() {
    bins1_.clear();
    bins2_.clear();
  }

  void prepare(Elem<Ksize, Vsize> &&elem) {
    bins1_.push_back(elem);
  }

  void build(const PrfKey &prf_key, unsigned jobs);
  bool lookup(Elem<Ksize, Vsize> &query, const PrfKey &prf_key);

  size_t size() const noexcept {
    return bins1_.size() + bins2_.size();
  }
  bool empty() const noexcept {
    return size() == 0;
  }

 private:
  void build_pass(std::vector<Elem<Ksize, Vsize>> &bins, std::vector<Elem<Ksize, Vsize>> *overflow_pile,
    const PrfKey &prf_key, unsigned jobs);
  bool lookup_bin(typename std::vector<Elem<Ksize, Vsize>>::iterator bin_begin,
    typename std::vector<Elem<Ksize, Vsize>>::iterator bin_end, Elem<Ksize, Vsize> &query);
};

}  // namespace oht

// Impl
namespace oht {

using namespace oht::decl;

template <size_t Ksize, size_t Vsize>
void Oht<Ksize, Vsize>::build(const PrfKey &prf_key, unsigned jobs) {
  std::vector prf_key_buf(prf_key);
  prf_key_buf.push_back(0);

  prf_key_buf[prf_key_buf.size() - 1] = 1;
  build_pass(bins1_, &bins2_, prf_key_buf, jobs);

  for (auto &elem : bins2_) {
    elem.tag = tag::unset(elem.tag, tag::kTagExcess);
  }

  prf_key_buf[prf_key_buf.size() - 1] = 2;
  build_pass(bins2_, nullptr, prf_key_buf, jobs);
}

template <size_t Ksize, size_t Vsize>
void Oht<Ksize, Vsize>::build_pass(std::vector<Elem<Ksize, Vsize>> &bins,
  std::vector<Elem<Ksize, Vsize>> *overflow_pile, const PrfKey &prf_key, unsigned jobs) {
  // Assign bin index
  for (auto &elem : bins) {
    const auto bin_id = PrfInt(prf_key, elem.key, Ksize, kB_);
    elem.bin_id = bin_id;
  }

  // Add fillers
  for (auto i = 0; i < kB_; i++) {
    Elem<Ksize, Vsize> filler;
    filler.tag = tag::kTagFiller;
    filler.bin_id = i;
    for (auto j = 0; j < kZ_; j++) {
      bins.push_back(filler);
    }
  }

  // 1st osort to bins with fillers in the back of the bin
  const auto cmp1 = +[](const Elem<Ksize, Vsize> &a, const Elem<Ksize, Vsize> &b) {
    uint64_t a_id = (uint64_t(a.bin_id) << 32) | tag::get(a.tag, tag::kTagFiller);
    uint64_t b_id = (uint64_t(b.bin_id) << 32) | tag::get(b.tag, tag::kTagFiller);
    return obl::OblLt(a_id, b_id);
  };
  obl::OblSort(bins.begin(), bins.end(), cmp1, int(jobs));

  // Assign excess
  size_t bin_elem_num = 0;
  uint32_t last_bin_id = 0;
  for (auto &elem : bins) {
    bin_elem_num = obl::OblChoose(obl::OblEq(last_bin_id, elem.bin_id), bin_elem_num + 1, size_t(1));
    last_bin_id = elem.bin_id;
    elem.tag = obl::OblChoose(obl::OblGt(bin_elem_num, kZ_), elem.tag | tag::kTagExcess, elem.tag);
  }

  // 2nd osort to bins with the equal size `z` and a overflow pile.
  // In the overflow pile, `(k, v)`/dummy are in front of fillers.
  // Not sure whether it is required for the overflow pile, but we take it since it does not cost much.
  const auto cmp2 = +[](const Elem<Ksize, Vsize> &a, const Elem<Ksize, Vsize> &b) {
    const auto a_id1 = tag::get(a.tag, tag::kTagExcess);
    const auto b_id1 = tag::get(b.tag, tag::kTagExcess);
    const auto id1_ord = obl::OblLt(a_id1, b_id1);
    const auto id1_eq = obl::OblEq(a_id1, b_id1);

    const auto id2_excess = obl::OblGt(a.tag & tag::kTagExcess, uint32_t(0));
    // What the ****, the paper actually means though dummy should be put at very end, the first level pred should be
    // excess only. Please do not put a non-first-level pred as the first rule...
    const uint32_t a_excess_id2 = (uint32_t(tag::get(a.tag, kTagDummy)) << 8) | tag::get(a.tag, tag::kTagFiller);
    const uint32_t b_excess_id2 = (uint32_t(tag::get(b.tag, kTagDummy)) << 8) | tag::get(b.tag, tag::kTagFiller);
    const auto id2_excess_ord = obl::OblLt(a_excess_id2, b_excess_id2);
    const auto id2_non_excess_ord = obl::OblLt(a.bin_id, b.bin_id);
    const auto id2_ord = obl::OblChoose(id2_excess, id2_excess_ord, id2_non_excess_ord);

    return obl::OblChoose(id1_eq, id2_ord, id1_ord);
  };
  obl::OblSort(bins.begin(), bins.end(), cmp2, int(jobs));

  // Separate bins and the overflow pile
  if (overflow_pile != nullptr) {
    overflow_pile->reserve(bins.size() - kB_ * kZ_);
    overflow_pile->insert(overflow_pile->end(), bins.begin() + kB_ * kZ_, bins.end());
  }
  bins.resize(kB_ * kZ_);
  bins.shrink_to_fit();
}

template <size_t Ksize, size_t Vsize>
bool Oht<Ksize, Vsize>::lookup(Elem<Ksize, Vsize> &query, const PrfKey &prf_key) {
  std::vector prf_key_buf(prf_key);
  prf_key_buf.push_back(0);

  prf_key_buf[prf_key_buf.size() - 1] = 1;
  const auto bins1_bin_id = PrfInt(prf_key_buf, query.key, Ksize, kB_);
  const auto bins1_bin_begin = bins1_.begin() + bins1_bin_id * kZ_;
  const auto bins1_bin_end = bins1_bin_begin + kZ_;
  const auto bins1_found = lookup_bin(bins1_bin_begin, bins1_bin_end, query);
  std::array<uint8_t, Vsize> bins1_val;
  std::memcpy(bins1_val.data(), query.val, Vsize);

  prf_key_buf[prf_key_buf.size() - 1] = 2;
  const auto bins2_bin_id = PrfInt(prf_key_buf, query.key, Ksize, kB_);
  const auto bins2_bin_begin = bins2_.begin() + bins2_bin_id * kZ_;
  const auto bins2_bin_end = bins2_bin_begin + kZ_;
  const auto bins2_found = lookup_bin(bins2_bin_begin, bins2_bin_end, query);
  std::array<uint8_t, Vsize> bins2_val;
  std::memcpy(bins2_val.data(), query.val, Vsize);

  const auto found = bins1_found || bins2_found;
  std::array<uint8_t, Vsize> null_val;
  std::memcpy(null_val.data(), query.val, Vsize);
  auto val = null_val;
  val = obl::OblChoose(bins2_found, bins2_val, val);
  val = obl::OblChoose(bins1_found, bins1_val, val);
  const auto query_dummy = obl::OblGt(query.tag & kTagDummy, uint32_t(0));
  val = obl::OblChoose(query_dummy, null_val, val);
  std::memcpy(query.val, val.data(), Vsize);
  return found;
}

template <size_t Ksize, size_t Vsize>
bool Oht<Ksize, Vsize>::lookup_bin(typename std::vector<Elem<Ksize, Vsize>>::iterator bin_begin,
  typename std::vector<Elem<Ksize, Vsize>>::iterator bin_end, Elem<Ksize, Vsize> &query) {
  auto found = false;
  for (auto elem = bin_begin; elem != bin_end; elem++) {
    const auto key_eq = obl::OblEqArr(elem->key, query.key, Ksize);
    const auto dummy = obl::OblGt(elem->tag & kTagDummy, uint32_t(0));
    const auto filler = obl::OblGt(elem->tag & tag::kTagFiller, uint32_t(0));
    const auto found_here = key_eq && !dummy && !filler;
    found |= found_here;
    std::array<uint8_t, Vsize> elem_val;
    std::memcpy(elem_val.data(), elem->val, Vsize);
    std::array<uint8_t, Vsize> null_val;
    std::memcpy(null_val.data(), query.val, Vsize);
    auto val = obl::OblChoose(found_here, elem_val, null_val);
    std::memcpy(query.val, val.data(), Vsize);
  }
  return found;
}

}  // namespace oht
