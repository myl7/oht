#[cxx::bridge]
mod ffi {
    // Required to be put here because we will copy it in C++ (for obliviousness).
    // Copy derive is also required.
    #[derive(Debug, Copy, Clone)]
    struct Elem {
        pub tag: u32,
        pub key: [u8; KEY_SIZE],
        pub val: [u8; VAL_SIZE],
    }

    extern "Rust" {}

    extern "C++" {
        include!("oht/src/obl/mod.h");

        unsafe fn osort(vec: &mut [Elem], cmp: fn(&Elem, &Elem) -> bool, jobs: usize);
    }
}
