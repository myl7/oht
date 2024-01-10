// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

//! `n` is the elem num. See [`oht`] for `b` and `z`.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use oht::{Elem, Oht, KEY_SIZE, VAL_SIZE};
use rand::prelude::*;

const PRF_KEY: &[u8; 32] = b"01234567890123456789012345678901";

pub fn bench(c: &mut Criterion) {
    const N: usize = 100_000;
    let elems = (0..N).map(|i| {
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
        thread_rng().fill_bytes(&mut elem.key[4..]);
        thread_rng().fill_bytes(&mut elem.val);
        elem
    });
    // const Z: usize = (5f64 * 3.27).ceil() as usize;
    // const B: u16 = ((N as f64) / 3.27).ceil() as u16;
    const B: u16 = 30582;
    const Z: usize = 17;
    c.bench_with_input(
        BenchmarkId::new("oht build n=100k", format!("b={B} z={Z}")),
        &(),
        |bencher, &()| {
            bencher.iter(|| {
                let mut oht = Oht::<B, Z>::new();
                oht.bins1.extend(elems.clone());
                oht.build(PRF_KEY, 10);
            })
        },
    );
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench
}
criterion_main!(benches);
