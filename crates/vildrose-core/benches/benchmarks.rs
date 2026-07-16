//! Criterion benchmarks for `vildrose-core`.

mod word;

use criterion::criterion_main;

criterion_main!(
    word::layout::benches,
    word::trit_packing::benches,
    word::arithmetics::benches,
);
