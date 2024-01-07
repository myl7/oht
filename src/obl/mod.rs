// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

pub mod tag;

pub use ffi::*;
use std::fmt;
use tag::prelude::*;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/include/obl/ffi.rs"));

impl fmt::Debug for Elem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Elem")
            .field("tag", &self.tag)
            .field("tag&dummy", &tag::get(self.tag, TAG_DUMMY))
            .field("tag&filler", &tag::get(self.tag, TAG_FILLER))
            .field("tag&excess", &tag::get(self.tag, TAG_EXCESS))
            .field("tag&bin_idx", &tag::get(self.tag, TAG_BIN_IDX))
            .field("key", &self.key)
            .field("val.len", &self.val.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let mut vec = vec![
            Elem {
                tag: 0,
                key: std::array::from_fn(|_| 0),
                val: std::array::from_fn(|_| 0)
            };
            3
        ];
        vec[0].key[0] = 3;
        vec[1].key[0] = 2;
        vec[2].key[0] = 1;
        osort(&mut vec, |a, b| a.key[0] < b.key[0], 1);
        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0].key[0], 1);
        assert_eq!(vec[1].key[0], 2);
        assert_eq!(vec[2].key[0], 3);
    }
}
