// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

//! Linux-style bit tag
//!
//! - `x | tag`: set the tag.
//! - `x & !tag`: unset the tag.
//! - `x & tag > 0`: check the tag is set.
//! - `x & tag == 0`: check the tag is unset.
//! - `(x & tag) >> tag.trailing_zeros()`: get the tag value.
//! - `x | (((y as typeof(x)) << tag.trailing_zeros()) & tag)`: set the tag value.
//! - `x | ((y as typeof(x)) << tag.trailing_zeros())`: set the tag value if y is known to fit the size.
//!
//! There are also helpers for these operations

pub use prelude::*;

pub mod prelude {
    pub const TAG_DUMMY: u32 = 1 << 0;
    pub(crate) const TAG_FILLER: u32 = 1 << 1;
    pub(crate) const TAG_EXCESS: u32 = 1 << 2;
    pub(crate) const TAG_BIN_IDX: u32 = 0xffff << 16;
}

pub fn init() -> u32 {
    0
}

pub fn set(x: u32, tag: u32) -> u32 {
    x | tag
}

pub fn unset(x: u32, tag: u32) -> u32 {
    x & !tag
}

pub fn check(x: u32, tag: u32) -> bool {
    x & tag > 0
}

pub fn check_not(x: u32, tag: u32) -> bool {
    x & tag == 0
}

pub fn get(x: u32, tag: u32) -> u32 {
    (x & tag) >> tag.trailing_zeros()
}

pub fn set_val(x: u32, tag: u32, val: u32) -> u32 {
    x | ((val << tag.trailing_zeros()) & tag)
}

pub fn set_val_fit(x: u32, tag: u32, val: u32) -> u32 {
    x | (val << tag.trailing_zeros())
}
