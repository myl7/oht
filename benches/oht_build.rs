use criterion::{criterion_group, criterion_main, Criterion};
use oht::{Elem, Oht};
use rand::prelude::*;

const PRF_KEY: &[u8; 32] = b"01234567890123456789012345678901";

pub fn bench(c: &mut Criterion) {
    let elems = (0..100_000).map(|i| {
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
        thread_rng().fill_bytes(&mut elem.key[4..]);
        thread_rng().fill_bytes(&mut elem.val);
        elem
    });

    c.bench_function("oht build 100k elems", |b| {
        b.iter(|| {
            let mut oht = Oht::new(4, 25_000);
            oht.build(elems.clone(), PRF_KEY, 1);
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench
}
criterion_main!(benches);
