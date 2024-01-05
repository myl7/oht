// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

pub use ffi::*;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/include/obl/ffi.rs"));

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
        unsafe { osort(&mut vec, |a, b| a.key[0] < b.key[0], 1) };
        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0].key[0], 1);
        assert_eq!(vec[1].key[0], 2);
        assert_eq!(vec[2].key[0], 3);
    }
}
