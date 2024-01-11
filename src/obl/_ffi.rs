// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

pub const KEY_SIZE: usize = { KEY_SIZE };
pub const VAL_SIZE: usize = { VAL_SIZE };

#[cxx::bridge]
mod ffi {
    // Required to be put here because we will copy it in C++ (for obliviousness).
    // Copy derive is also required.
    #[derive(Copy, Clone)]
    struct Elem {
        pub tag: u32,
        pub key: [u8; { KEY_SIZE }],
        pub val: [u8; { VAL_SIZE }],
    }

    extern "Rust" {}

    unsafe extern "C++" {
        include!("oht/src/obl/mod.h");

        fn osort(vec: &mut [Elem], cmp: fn(&Elem, &Elem) -> bool, jobs: usize);
        fn olt(a: u32, b: u32) -> bool;
        fn ogt(a: u32, b: u32) -> bool;
        fn oeq(a: u32, b: u32) -> bool;
        fn oeq_key(a: [u8; { KEY_SIZE }], b: [u8; { KEY_SIZE }]) -> bool;
        fn ochoose_u32(pred: bool, a: u32, b: u32) -> u32;
        fn ochoose_bool(pred: bool, a: bool, b: bool) -> bool;
        pub fn ochoose_val(
            pred: bool,
            a: [u8; { VAL_SIZE }],
            b: [u8; { VAL_SIZE }],
        ) -> [u8; { VAL_SIZE }];
    }
}
