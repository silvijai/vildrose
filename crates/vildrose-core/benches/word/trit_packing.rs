//! Benchmarks for trit packing, seeing whether 1, 4, or 5 trits per byte are optimal.
//! Both in terms of memory usage and performance.

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use vildrose_core::trit::Trit;

#[allow(clippy::wildcard_imports)]
use super::common::packed_trits::*;

// TODO:
// Measure memory saved more explicitly for different layouts.

fn assert_tables_match_division() {
    for byte in 0u8..243 {
        assert_eq!(
            decode_5_table(&[byte], 5),
            decode_5_div(&[byte], 5),
            "5-trit table differs for byte {byte}",
        );
    }

    for byte in 243u8..=u8::MAX {
        assert_eq!(
            DECODE_5_TABLE[usize::from(byte)],
            INVALID,
            "invalid 5-trit byte {byte} has a table entry",
        );
    }
}

fn assert_round_trips() {
    for length in 0..64 {
        let trits = input(length);

        let encoded_1 = encode_1(&trits);
        let encoded_4 = encode_4(&trits);
        let encoded_5 = encode_5(&trits);

        assert_eq!(decode_1_direct(&encoded_1, length), trits);
        assert_eq!(decode_4_direct(&encoded_4, length), trits);
        assert_eq!(decode_5_div(&encoded_5, length), trits);
        assert_eq!(decode_5_table(&encoded_5, length), trits);

        assert_eq!(encoded_1.len(), length);
        assert_eq!(encoded_4.len(), length.div_ceil(4));
        assert_eq!(encoded_5.len(), length.div_ceil(5));
    }
}

// <- Criterion stuff starts here
fn input(length: usize) -> Vec<Trit> {
    (0..length)
        .map(|index| match (index * 17 + 11) % 3 {
            0 => Trit::N,
            1 => Trit::Z,
            _ => Trit::P,
        })
        .collect()
}

fn benchmark_trit_packing(c: &mut Criterion) {
    assert_round_trips();
    assert_tables_match_division();

    let mut group = c.benchmark_group("trit packing");

    for length in [9, 27, 54, 243, 1_024, 65_536] {
        let trits = input(length);
        let encoded_1 = encode_1(&trits);
        let encoded_4 = encode_4(&trits);
        let encoded_5 = encode_5(&trits);

        _ = group.throughput(Throughput::Elements(length as u64));

        _ = group.bench_with_input(BenchmarkId::new("1/encode", length), &trits, |b, trits| {
            b.iter(|| encode_1(black_box(trits)));
        });

        _ = group.bench_with_input(BenchmarkId::new("4/encode", length), &trits, |b, trits| {
            b.iter(|| encode_4(black_box(trits)));
        });

        _ = group.bench_with_input(BenchmarkId::new("5/encode", length), &trits, |b, trits| {
            b.iter(|| encode_5(black_box(trits)));
        });

        _ = group.bench_with_input(
            BenchmarkId::new("1/decode-direct", length),
            &encoded_1,
            |b, bytes| b.iter(|| decode_1_direct(black_box(bytes), length)),
        );

        _ = group.bench_with_input(
            BenchmarkId::new("4/decode-direct", length),
            &encoded_4,
            |b, bytes| b.iter(|| decode_4_direct(black_box(bytes), length)),
        );

        _ = group.bench_with_input(
            BenchmarkId::new("5/decode-div", length),
            &encoded_5,
            |b, bytes| b.iter(|| decode_5_div(black_box(bytes), length)),
        );

        _ = group.bench_with_input(
            BenchmarkId::new("5/decode-table", length),
            &encoded_5,
            |b, bytes| b.iter(|| decode_5_table(black_box(bytes), length)),
        );
    }

    group.finish();
}

criterion_group!(benches, benchmark_trit_packing);
criterion_main!(benches);
