//! Benchmarks for word layout and trit packing, especially in relation
//! to word-level operations.

use std::hint::black_box;

use core::fmt;
use core::ops::Neg;
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use vildrose_core::trit::Trit;

#[allow(clippy::wildcard_imports)]
use super::common::packed_trits::*;

// TODO: look into using a larger byte array, and using the whole length to fit trits
// So instead of 5 trits in one byte, it could be 27 trits in a u64

// <- Basic word structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word<const N: usize>(pub [Trit; N]);

impl<const N: usize> Word<N> {
    pub fn negate(self) -> Self {
        Self(self.0.map(Trit::negate))
    }

    pub fn sign(self) -> Trit {
        self.0
            .iter()
            .rev()
            .copied()
            .find(|&t| t != Trit::Z)
            .unwrap_or(Trit::Z)
    }

    pub fn abs(self) -> Self {
        if self.sign() == Trit::N {
            self.negate()
        } else {
            self
        }
    }
}

impl<const N: usize> Neg for Word<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl<const N: usize> fmt::Display for Word<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for trit in self.0.iter().rev() {
            write!(f, "{trit}")?;
        }

        Ok(())
    }
}

// <- Word byte storage
#[derive(Clone)]
pub struct Word1<const N: usize> {
    bytes: [u8; N],
}

#[derive(Clone)]
pub struct Word4<const N: usize> {
    bytes: Vec<u8>,
}

#[derive(Clone)]
pub struct Word5<const N: usize> {
    bytes: Vec<u8>,
}

// <- Word encoding
pub trait WordEncoding<const N: usize>: Clone {
    fn encode(word: &Word<N>) -> Self;
    fn decode(&self) -> Word<N>;
    fn layout_name() -> &'static str;
}

impl<const N: usize> WordEncoding<N> for Word1<N> {
    fn encode(word: &Word<N>) -> Self {
        let bytes = encode_1(&word.0);
        Self {
            bytes: bytes.try_into().unwrap(),
        }
    }

    fn decode(&self) -> Word<N> {
        let trits = decode_1_direct(&self.bytes, N);
        Word(trits.try_into().unwrap())
    }

    fn layout_name() -> &'static str {
        "1"
    }
}

impl<const N: usize> WordEncoding<N> for Word4<N> {
    fn encode(word: &Word<N>) -> Self {
        Self {
            bytes: encode_4(&word.0),
        }
    }

    fn decode(&self) -> Word<N> {
        let trits = decode_4_direct(&self.bytes, N);
        Word(trits.try_into().unwrap())
    }

    fn layout_name() -> &'static str {
        "4"
    }
}

impl<const N: usize> WordEncoding<N> for Word5<N> {
    fn encode(word: &Word<N>) -> Self {
        Self {
            bytes: encode_5(&word.0),
        }
    }

    fn decode(&self) -> Word<N> {
        let trits = decode_5_div(&self.bytes, N);
        Word(trits.try_into().unwrap())
    }

    fn layout_name() -> &'static str {
        "5"
    }
}

fn input_word<const N: usize>() -> Word<N> {
    Word(std::array::from_fn(|index| match (index * 17 + 11) % 3 {
        0 => Trit::N,
        1 => Trit::Z,
        _ => Trit::P,
    }))
}

fn bench_word_layout_for<const N: usize, E: WordEncoding<N>>(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
) {
    let word = input_word::<N>();
    let encoded = E::encode(&word);

    _ = group.throughput(Throughput::Elements(N as u64));

    _ = group.bench_function(
        BenchmarkId::new(format!("{}/encode", E::layout_name()), N),
        |b| b.iter(|| E::encode(black_box(&word))),
    );

    _ = group.bench_function(
        BenchmarkId::new(format!("{}/decode", E::layout_name()), N),
        |b| b.iter(|| black_box(&encoded).decode()),
    );

    _ = group.bench_function(
        BenchmarkId::new(format!("{}/negate", E::layout_name()), N),
        |b| {
            b.iter(|| {
                let decoded = black_box(&encoded).decode();
                let result = decoded.negate();
                black_box(E::encode(&result))
            });
        },
    );

    _ = group.bench_function(
        BenchmarkId::new(format!("{}/sign", E::layout_name()), N),
        |b| {
            b.iter(|| {
                let decoded = black_box(&encoded).decode();
                black_box(decoded.sign())
            });
        },
    );

    _ = group.bench_function(
        BenchmarkId::new(format!("{}/abs", E::layout_name()), N),
        |b| {
            b.iter(|| {
                let decoded = black_box(&encoded).decode();
                let result = decoded.abs();
                black_box(E::encode(&result))
            });
        },
    );

    _ = group.bench_function(
        BenchmarkId::new(format!("{}/fmt", E::layout_name()), N),
        |b| {
            b.iter(|| {
                let decoded = black_box(&encoded).decode();
                black_box(decoded.to_string())
            });
        },
    );
}

fn benchmark_word_layout(c: &mut Criterion) {
    let mut group = c.benchmark_group("word layout");

    bench_word_layout_for::<3, Word1<3>>(&mut group);
    bench_word_layout_for::<3, Word4<3>>(&mut group);
    bench_word_layout_for::<3, Word5<3>>(&mut group);

    bench_word_layout_for::<9, Word1<9>>(&mut group);
    bench_word_layout_for::<9, Word4<9>>(&mut group);
    bench_word_layout_for::<9, Word5<9>>(&mut group);

    bench_word_layout_for::<27, Word1<27>>(&mut group);
    bench_word_layout_for::<27, Word4<27>>(&mut group);
    bench_word_layout_for::<27, Word5<27>>(&mut group);

    bench_word_layout_for::<54, Word1<54>>(&mut group);
    bench_word_layout_for::<54, Word4<54>>(&mut group);
    bench_word_layout_for::<54, Word5<54>>(&mut group);

    bench_word_layout_for::<108, Word1<108>>(&mut group);
    bench_word_layout_for::<108, Word4<108>>(&mut group);
    bench_word_layout_for::<108, Word5<108>>(&mut group);

    group.finish();
}

criterion_group!(benches, benchmark_word_layout);
criterion_main!(benches);
