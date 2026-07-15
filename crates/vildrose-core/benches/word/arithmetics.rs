//! Benchmarks for word arithmetic, focusing on performance between native ternary and int based arithmetic.

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use vildrose_core::word::Word27;

const SAMPLE_COUNT: usize = 1024;

fn sample_word27s() -> Vec<Word27> {
    // Reuse your Tryte-range sampling if you like; here use a simple deterministic pattern.
    let mut v = Vec::with_capacity(SAMPLE_COUNT);
    for i in 0..SAMPLE_COUNT {
        // Map i into some balanced range, then into Word27.
        #[allow(clippy::cast_possible_wrap)]
        let n = ((i as i64) * 123_457).rem_euclid(2 * Word27::MAX_INT + 1) - Word27::MAX_INT;
        v.push(Word27::from_int(n).expect("generated value must fit Word27"));
    }
    v
}

fn wrap_balanced_i64(value: i128, trit_count: u32) -> i64 {
    let modulus = 3_i128.pow(trit_count);
    let half_range = (modulus - 1) / 2;
    let wrapped = (value + half_range).rem_euclid(modulus) - half_range;
    i64::try_from(wrapped).expect("wrapped value must fit i64")
}

fn benchmark_word27_add_vs_int(c: &mut Criterion) {
    let mut group = c.benchmark_group("word27_add_vs_int");

    let words = sample_word27s();

    // Precompute integer view to avoid including to_int() in the pure-int baseline.
    let ints: Vec<i64> = words.iter().map(|w| w.to_int()).collect();

    _ = group.throughput(Throughput::Elements(words.len() as u64));

    // Native Word27 addition: acc = acc + x
    _ = group.bench_function(BenchmarkId::new("word27_add_native", SAMPLE_COUNT), |b| {
        b.iter(|| {
            let mut acc = Word27::zero();
            for &x in &words {
                acc = black_box(acc + x);
            }
            black_box(acc)
        });
    });

    // Integer addition: acc = acc + x, using i64
    _ = group.bench_function(BenchmarkId::new("i64_add", SAMPLE_COUNT), |b| {
        b.iter(|| {
            let mut acc: i64 = 0;
            for &x in &ints {
                acc = black_box(acc.wrapping_add(x));
            }
            black_box(acc)
        });
    });

    // Convert to int, add, convert back each step (slow path)
    let _ = group.bench_function(
        BenchmarkId::new("word27_add_via_int_roundtrip", SAMPLE_COUNT),
        |b| {
            b.iter(|| {
                let mut acc = Word27::zero();
                for &x in &words {
                    let acc_int = acc.to_int();
                    let x_int = x.to_int();
                    // Do the addition in a wider type, then wrap back to the balanced range.
                    let sum_int = wrap_balanced_i64(i128::from(acc_int) + i128::from(x_int), 27);
                    acc =
                        black_box(Word27::from_int(sum_int).expect("wrapped sum must fit Word27"));
                }
                black_box(acc)
            });
        },
    );

    group.finish();
}

fn benchmark_word27_sub_vs_int(c: &mut Criterion) {
    let mut group = c.benchmark_group("word27_sub_vs_int");

    let words = sample_word27s();
    let ints: Vec<i64> = words.iter().map(|w| w.to_int()).collect();

    _ = group.throughput(Throughput::Elements(words.len() as u64));

    _ = group.bench_function(BenchmarkId::new("word27_sub_native", SAMPLE_COUNT), |b| {
        b.iter(|| {
            let mut acc = Word27::zero();
            for &x in &words {
                acc = black_box(acc - x);
            }
            black_box(acc)
        });
    });

    _ = group.bench_function(BenchmarkId::new("i64_sub", SAMPLE_COUNT), |b| {
        b.iter(|| {
            let mut acc: i64 = 0;
            for &x in &ints {
                acc = black_box(acc.wrapping_sub(x));
            }
            black_box(acc)
        });
    });

    let _ = group.bench_function(
        BenchmarkId::new("word27_sub_via_int_roundtrip", SAMPLE_COUNT),
        |b| {
            b.iter(|| {
                let mut acc = Word27::zero();
                for &x in &words {
                    let acc_int = acc.to_int();
                    let x_int = x.to_int();
                    let diff_int = wrap_balanced_i64(i128::from(acc_int) - i128::from(x_int), 27);
                    acc = black_box(
                        Word27::from_int(diff_int).expect("wrapped diff must fit Word27"),
                    );
                }
                black_box(acc)
            });
        },
    );

    group.finish();
}

fn benchmark_arithmetics(c: &mut Criterion) {
    benchmark_word27_add_vs_int(c);
    benchmark_word27_sub_vs_int(c);
}

criterion_group!(benches, benchmark_arithmetics);
criterion_main!(benches);
