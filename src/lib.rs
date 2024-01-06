// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

//! We always use `b` for the bin number, `z` for the bin capacity

mod crypto;
mod obl;

pub use obl::Elem;

/// As a 2-tier hash table, `h1` is for the 1st tier and `h2` is for the 2nd tier.
/// Every tier is some bins, which an index can be used to lookup the bin.
pub struct Oht {
    bins: Vec<Elem>,
    overflow: Vec<Elem>,
    b: u16,
    z: usize,
}

impl Oht {
    /// See [`self`] for the meaning of `b` and `z`
    pub fn new(b: u16, z: usize) -> Oht {
        Oht {
            bins: vec![],
            overflow: vec![],
            b,
            z,
        }
    }

    pub fn clear(&mut self) {
        self.bins.clear();
        self.overflow.clear();
    }

    pub fn build(&mut self, elems: impl IntoIterator<Item = Elem>, prf_key: &[u8], jobs: usize) {
        self.clear();

        for mut elem in elems {
            let bin_idx = crypto::prf_pow2(prf_key, &elem.key, self.b);
            elem.tag &= (bin_idx as u32) << 16;
            self.bins.push(elem);
        }
        (0..self.b).for_each(|i| {
            let filler = Elem {
                key: crypto::prf(prf_key, &i.to_le_bytes()),
                val: [0; 256],
                tag: ((i as u32) << 16) | TAG_FILLER,
            };
            (0..self.z).for_each(|_| {
                self.bins.push(filler.clone());
            });
        });
        obl::osort(
            &mut self.bins,
            |a, b| {
                let a_id = (a.tag & TAG_BIN_IDX) | (a.tag & TAG_FILLER);
                let b_id = (b.tag & TAG_BIN_IDX) | (b.tag & TAG_FILLER);
                obl::olt(a_id, b_id)
            },
            jobs,
        );
        let mut bin_elem_num = vec![0; self.b as usize];
        for elem in self.bins.iter_mut() {
            let bin_idx = ((elem.tag & TAG_BIN_IDX) >> TAG_BIN_IDX.trailing_zeros()) as usize;
            bin_elem_num[bin_idx] += 1;
            elem.tag = obl::ochoose_u32(
                obl::ogt(bin_elem_num[bin_idx], self.z as u32),
                elem.tag & TAG_EXCESS,
                elem.tag,
            );
        }
        obl::osort(
            &mut self.bins,
            |a, b| {
                let a_id1 = ((a.tag & TAG_DUMMY) >> TAG_DUMMY.trailing_zeros() << 1)
                    | ((a.tag & TAG_EXCESS) >> TAG_EXCESS.trailing_zeros());
                let b_id1 = ((b.tag & TAG_DUMMY) >> TAG_DUMMY.trailing_zeros() << 1)
                    | ((b.tag & TAG_EXCESS) >> TAG_EXCESS.trailing_zeros());
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
        for elem in self
            .bins
            .splice((self.b as usize * self.z)..self.bins.len(), [])
        {
            if self.overflow.len() < self.b as usize * self.z {
                self.overflow.push(elem);
            } else {
                panic!("overflow");
            }
        }
    }
}

pub const TAG_DUMMY: u32 = 1 << 0;
const TAG_FILLER: u32 = 1 << 1;
const TAG_EXCESS: u32 = 1 << 2;
const TAG_BIN_IDX: u32 = 0xffff << 16;

#[cfg(test)]
mod tests {
    use super::*;

    const PRF_KEY: &[u8; 32] = b"01234567890123456789012345678901";

    #[test]
    fn test_oht_build_ok() {
        let mut oht = Oht::new(4, 100);
        let elems = (0..100).map(|i| {
            let mut elem = Elem {
                key: [0; 32],
                val: [0; 256],
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
        });
        oht.build(elems, PRF_KEY, 1);
        // TODO: Check elem exists
    }

    #[test]
    fn test_oht_build_mt_ok() {
        let mut oht = Oht::new(4, 1300);
        let elems = (0..5000).map(|i| {
            let mut elem = Elem {
                key: [0; 32],
                val: [0; 256],
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
        });
        oht.build(elems, PRF_KEY, 4);
        // TODO: Check elem exists
    }
}
