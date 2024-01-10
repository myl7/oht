// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

//! We always use `b` for the bin number, `z` for the bin capacity

mod crypto;
mod obl;

pub use obl::tag;
use obl::tag::{prelude::*, set_val_fit};
pub use obl::{Elem, KEY_SIZE, VAL_SIZE};
pub use tag::TAG_DUMMY;

/// As a 2-tier hash table, `h1` is for the 1st tier and `h2` is for the 2nd tier.
/// Every tier is some bins, which an index can be used to lookup the bin.
/// See [`crate`] for `b` and `z`.
#[derive(Debug, Clone)]
pub struct Oht {
    /// `$H_1$`. The 1st tier OHT. Built first.
    pub bins1: Vec<Elem>,
    /// `$H_2$`. The 2nd tier OHT. Built second.
    pub bins2: Vec<Elem>,
    pub b: u16,
    pub z: usize,
}

impl Oht {
    pub fn new(b: u16, z: usize) -> Self {
        Oht {
            bins1: Vec::with_capacity(b as usize * z),
            bins2: Vec::with_capacity(b as usize * z),
            b,
            z,
        }
    }

    pub fn clear(&mut self) {
        self.bins1.clear();
        self.bins2.clear();
    }

    /// Put elements to the preallocated buffer for preparation
    pub fn prepare(&mut self, elems: impl IntoIterator<Item = Elem>) {
        self.clear();
        self.bins1.extend(elems);
    }

    /// `$Build(1^{\lambda}, \{(k_i, v_i)|dummy\}_{i \in [n]})$`.
    /// Build the OHT from the given elements.
    /// This does not do encryption, so do it after calling this.
    pub fn build(&mut self, prf_key: &[u8], jobs: usize) {
        let mut prf_key_buf = [&[0], prf_key].concat();

        prf_key_buf[0] = 1;
        Self::build_pass(
            (self.b, self.z),
            &mut self.bins1,
            &prf_key_buf,
            jobs,
            Some(&mut self.bins2),
        );

        for elem in self.bins2.iter_mut() {
            elem.tag = tag::unset(elem.tag, TAG_EXCESS);
        }

        prf_key_buf[0] = 2;
        Self::build_pass((self.b, self.z), &mut self.bins2, &prf_key_buf, jobs, None);
    }

    /// Build pass executed twice in one build.
    /// Exported to ease utilization.
    /// Returns the bins and overflow pile.
    /// If `collect_overflow` is false, the overflow pile is always `None` (dropped), otherwise returned.
    pub fn build_pass(
        (b, z): (u16, usize),
        bins: &mut Vec<Elem>,
        prf_key: &[u8],
        jobs: usize,
        overflow_pile: Option<&mut Vec<Elem>>,
    ) {
        // Assign bin index
        for elem in bins.iter_mut() {
            let bin_idx = crypto::prf_int(prf_key, &elem.key, b);
            elem.tag = set_val_fit(elem.tag, TAG_BIN_IDX, bin_idx as u32);
        }

        // Add fillers
        (0..b).for_each(|i| {
            let filler = Elem {
                key: crypto::prf(prf_key, &i.to_le_bytes()),
                val: [0; 256],
                tag: tag::set_val_fit(tag::init() | TAG_FILLER, TAG_BIN_IDX, i as u32),
            };
            (0..z).for_each(|_| {
                bins.push(filler);
            });
        });

        // 1st osort to bins with fillers in the back of the bin.
        obl::osort(
            bins,
            |a, b| {
                // Tag bit positions are considered
                let a_id = a.tag & (TAG_BIN_IDX | TAG_FILLER);
                let b_id = b.tag & (TAG_BIN_IDX | TAG_FILLER);
                obl::olt(a_id, b_id)
            },
            jobs,
        );

        // Assign excess
        let mut bin_elem_num = 0;
        let mut last_bin_idx = 0;
        for elem in bins.iter_mut() {
            let bin_idx = tag::get(elem.tag, TAG_BIN_IDX);
            bin_elem_num = obl::ochoose_u32(obl::oeq(last_bin_idx, bin_idx), bin_elem_num + 1, 1);
            last_bin_idx = bin_idx;
            elem.tag = obl::ochoose_u32(
                obl::ogt(bin_elem_num, z as u32),
                elem.tag | TAG_EXCESS,
                elem.tag,
            );
        }

        // 2nd osort to bins with the equal size `z` and a overflow pile.
        // In the overflow pile, `(k, v)`/dummy are in front of fillers.
        // Not sure whether it is required for the overflow pile, but we take it since it does not cost much.
        obl::osort(
            bins,
            |a, b| {
                // Tag bit len is considered
                let a_id1 = (tag::get(a.tag, TAG_DUMMY) << 1) | tag::get(a.tag, TAG_EXCESS);
                let b_id1 = (tag::get(b.tag, TAG_DUMMY) << 1) | tag::get(b.tag, TAG_EXCESS);
                let id1_ord = obl::olt(a_id1, b_id1);
                let id1_eq = obl::oeq(a_id1, b_id1);

                let id2_excess = obl::ogt(a.tag & TAG_EXCESS, 0);
                let id2_excess_ord = obl::olt(a.tag & TAG_FILLER, b.tag & TAG_FILLER);
                let id2_non_excess_ord = obl::olt(a.tag & TAG_BIN_IDX, b.tag & TAG_BIN_IDX);
                let id2_ord = obl::ochoose_bool(id2_excess, id2_excess_ord, id2_non_excess_ord);

                obl::ochoose_bool(id1_eq, id2_ord, id1_ord)
            },
            jobs,
        );

        // Separate bins and the overflow pile
        if let Some(overflow_pile) = overflow_pile {
            overflow_pile.extend(bins.splice(b as usize * z.., []));
        } else {
            bins.truncate(b as usize * z);
        }
        bins.shrink_to_fit();
    }

    /// `$Lookup(T, k)$`.
    /// Lookup a key in the OHT.
    /// We reuse `val` field in `Elem` `query` to get the null value.
    /// Returns whether found and the found value (null when all not found or dummy).
    /// This does not do decryption, so do it before calling this.
    pub fn lookup(&self, query: Elem, prf_key: &[u8]) -> (bool, [u8; VAL_SIZE]) {
        let key = &query.key;
        let null_val = &query.val;
        let mut prf_key_buf = [&[0], prf_key].concat();
        let b = self.b;
        let z = self.z;

        prf_key_buf[0] = 1;
        let bins1_bin_idx = crypto::prf_int(&prf_key_buf, key, b);
        let bins1_bin_start = bins1_bin_idx as usize * z;
        let bins1_range = &self.bins1[bins1_bin_start..bins1_bin_start + z];
        let (bins1_found, bins1_val) = self.lookup_bin(bins1_range, &query);

        prf_key_buf[0] = 2;
        let bins2_bin_idx = crypto::prf_int(&prf_key_buf, key, b);
        let bins2_bin_start = bins2_bin_idx as usize * z;
        let bins2_range = &self.bins2[bins2_bin_start..bins2_bin_start + z];
        let (bins2_found, bins2_val) = self.lookup_bin(bins2_range, &query);

        let found = bins1_found | bins2_found;
        let val = *null_val;
        let val = obl::ochoose_val(bins2_found, bins2_val, val);
        let val = obl::ochoose_val(bins1_found, bins1_val, val);
        let query_dummy = obl::ogt(query.tag & TAG_DUMMY, 0);
        let val = obl::ochoose_val(query_dummy, *null_val, val);
        (found, val)
    }

    /// Exported to ease utilization.
    /// See [`Self::lookup`] for the null value, `query`, and returned values.
    pub fn lookup_bin(&self, bin_range: &[Elem], query: &Elem) -> (bool, [u8; VAL_SIZE]) {
        let mut found = false;
        let null_val = &query.val;
        let mut val = *null_val;
        for elem in bin_range.iter() {
            let key_eq = obl::oeq_key(elem.key, query.key);
            let elem_dummy = obl::ogt(elem.tag & TAG_DUMMY, 0);
            let elem_filler = obl::ogt(elem.tag & TAG_FILLER, 0);
            let found_here = key_eq & !elem_dummy & !elem_filler;
            found |= found_here;
            val = obl::ochoose_val(found_here, elem.val, val);
        }
        (found, val)
    }

    pub fn len(&self) -> usize {
        self.bins1.len() + self.bins2.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl IntoIterator for Oht {
    type Item = Elem;
    type IntoIter =
        std::iter::Chain<std::vec::IntoIter<Self::Item>, std::vec::IntoIter<Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        self.bins1.into_iter().chain(self.bins2.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    const PRF_KEY: &[u8; 32] = b"01234567890123456789012345678901";

    fn check_bin_elems(bin_range: &[Elem], bin_idx: u16) {
        for (i, elem) in bin_range.iter().enumerate() {
            assert!(elem.tag & TAG_EXCESS == 0);
            assert_eq!(
                tag::get(elem.tag, TAG_BIN_IDX),
                bin_idx as u32,
                "{:?} at bin {} idx {}",
                elem,
                bin_idx,
                i
            );
        }
    }

    fn check_bins(bins: &[Elem], b: u16, z: usize) {
        assert_eq!(bins.len(), b as usize * z);
        for i in 0..b {
            let bin_range = &bins[i as usize * z..(i as usize + 1) * z];
            check_bin_elems(bin_range, i);
        }
    }

    #[test]
    #[serial]
    fn test_oht_build_ok() {
        let mut oht = Oht::new(4, 100);
        let elems: Vec<_> = (0..100)
            .map(|i| {
                let mut elem = Elem {
                    key: [0; KEY_SIZE],
                    val: [0; VAL_SIZE],
                    tag: 0,
                };
                (i as u32)
                    .to_le_bytes()
                    .iter()
                    .enumerate()
                    .for_each(|(i, b)| {
                        elem.key[i] = *b;
                    });
                elem
            })
            .collect();
        oht.bins1 = elems.clone();
        oht.build(PRF_KEY, 1);
        check_bins(&oht.bins1, 4, 100);
        check_bins(&oht.bins2, 4, 100);
    }

    #[test]
    #[serial]
    fn test_oht_build_mt_ok() {
        let mut oht = Oht::new(4, 1300);
        let elems: Vec<_> = (0..5000)
            .map(|i| {
                let mut elem = Elem {
                    key: [0; KEY_SIZE],
                    val: [0; VAL_SIZE],
                    tag: 0,
                };
                (i as u32)
                    .to_le_bytes()
                    .iter()
                    .enumerate()
                    .for_each(|(i, b)| {
                        elem.key[i] = *b;
                    });
                elem
            })
            .collect();
        oht.bins1 = elems.clone();
        oht.build(PRF_KEY, 4);
        check_bins(&oht.bins1, 4, 1300);
        check_bins(&oht.bins2, 4, 1300);
    }

    #[test]
    #[serial]
    fn test_oht_build_then_lookup_ok() {
        let mut oht = Oht::new(31, 17);
        let elems: Vec<_> = (0..100)
            .map(|i| {
                let mut elem = Elem {
                    key: [0; KEY_SIZE],
                    val: [1; VAL_SIZE],
                    tag: 0,
                };
                (i as u32)
                    .to_le_bytes()
                    .iter()
                    .enumerate()
                    .for_each(|(i, b)| {
                        elem.key[i] = *b;
                    });
                elem
            })
            .collect();
        oht.bins1 = elems.clone();
        oht.build(PRF_KEY, 1);
        let null_val = [0; VAL_SIZE];
        [1, 14, 51, 4, 8, 10]
            .map(|i| (i, elems[i].key))
            .into_iter()
            .for_each(|(i, key)| {
                let (found, res_val) = oht.lookup(
                    Elem {
                        tag: 0,
                        key,
                        val: null_val,
                    },
                    PRF_KEY,
                );
                assert!(found);
                assert_eq!(res_val, [1; VAL_SIZE], "key {}", i);
            });
        let mut key = [0; KEY_SIZE];
        1919u32.to_le_bytes().iter().enumerate().for_each(|(i, b)| {
            key[i] = *b;
        });
        let (found, res_val) = oht.lookup(
            Elem {
                tag: 0,
                key,
                val: null_val,
            },
            PRF_KEY,
        );
        assert!(!found);
        assert_eq!(res_val, null_val);
    }
}
