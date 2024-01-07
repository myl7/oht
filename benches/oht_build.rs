// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

//! `n` is the elem num. See [`oht`] for `b` and `z`.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use oht::{Elem, Oht, KEY_SIZE, VAL_SIZE};
use rand::prelude::*;

const PRF_KEY: &[u8; 32] = b"01234567890123456789012345678901";

pub fn bench(c: &mut Criterion) {
    let n = 100_000;
    let elems = (0..n).map(|i| {
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
    let z = (5f64 * 3.27).ceil() as usize;
    let b = ((n as f64) / 3.27).ceil() as u16;
    c.bench_with_input(
        BenchmarkId::new("oht build n=100k", format!("b={b} z={z}")),
        &(b, z),
        |bencher, &(b, z)| {
            bencher.iter(|| {
                let mut oht = Oht::new(b, z);
                oht.build(elems.clone(), PRF_KEY, 10);
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
