// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

use std::{env, fs};

fn main() {
    let ffi_src = String::from_utf8(fs::read("src/obl/_ffi.rs").unwrap()).unwrap();
    let key_size: usize = env_usize("OHT_KEY_SIZE", 32);
    let val_size: usize = env_usize("OHT_VAL_SIZE", 256);
    let ffi_src = ffi_src.replace("KEY_SIZE", &key_size.to_string());
    let ffi_src = ffi_src.replace("VAL_SIZE", &val_size.to_string());
    fs::write("include/obl/ffi.rs", ffi_src).unwrap();
    println!("cargo:rerun-if-env-changed=OHT_KEY_SIZE");
    println!("cargo:rerun-if-env-changed=OHT_VAL_SIZE");

    let mut cc_build = cxx_build::bridge("include/obl/ffi.rs");
    #[cfg(feature = "avx2")]
    cc_build.define("USE_AVX2", None).flag("-mavx2");
    #[cfg(feature = "gpl")]
    cc_build.include(concat!(env!("CARGO_MANIFEST_DIR"), "/include"));
    cc_build
        .cpp(true)
        .file("src/obl/mod.cpp")
        .file("src/obl/par_obl_primitives.cpp")
        .compile("obl");
    println!("cargo:rerun-if-changed=src/obl");
}

fn env_usize(key: &str, default_val: usize) -> usize {
    match env::var(key) {
        Ok(val) => {
            if val.is_empty() {
                default_val
            } else {
                val.parse().unwrap()
            }
        }
        Err(_) => default_val,
    }
}
