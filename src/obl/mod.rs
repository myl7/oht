// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

pub use ffi::*;
use std::fmt;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/include/obl/ffi.rs"));

/// Linux-style bit tag
///
/// - `x | tag`: set the tag.
/// - `x & !tag`: unset the tag.
/// - `x & tag > 0`: check the tag is set.
/// - `x & tag == 0`: check the tag is unset.
/// - `(x & tag) >> tag.trailing_zeros()`: get the tag value.
/// - `x | (((y as typeof(x)) << tag.trailing_zeros()) & tag)`: set the tag value.
/// - `x | ((y as typeof(x)) << tag.trailing_zeros())`: set the tag value if y is known to fit the size.
pub mod tag {
    pub const TAG_DUMMY: u32 = 1 << 0;
    pub const TAG_FILLER: u32 = 1 << 1;
    pub const TAG_EXCESS: u32 = 1 << 2;
    pub const TAG_BIN_IDX: u32 = 0xffff << 16;
}

impl fmt::Debug for Elem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Elem")
            .field("tag", &self.tag)
            .field(
                "tag&dummy",
                &((self.tag & tag::TAG_DUMMY) >> tag::TAG_DUMMY.trailing_zeros()),
            )
            .field(
                "tag&filler",
                &((self.tag & tag::TAG_FILLER) >> tag::TAG_FILLER.trailing_zeros()),
            )
            .field(
                "tag&excess",
                &((self.tag & tag::TAG_EXCESS) >> tag::TAG_EXCESS.trailing_zeros()),
            )
            .field(
                "tag&bin_idx",
                &((self.tag & tag::TAG_BIN_IDX) >> tag::TAG_BIN_IDX.trailing_zeros()),
            )
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
