//! Benchmarks for word layout, seeing whether 1, 4, or 5 trits per byte are optimal. Both in terms of memory usage and performance.

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use vildrose_core::trit::Trit;

// TODO: Try setting this up with actual word sizes, more than a byte at a time, and figure out how much memory is saved by using different trit layouts

const fn trit_to_digit(trit: Trit) -> u8 {
    match trit {
        Trit::N => 0,
        Trit::Z => 1,
        Trit::P => 2,
    }
}

fn bits_to_trit(bits: u16) -> Trit {
    match bits {
        0b00 => Trit::N,
        0b01 => Trit::Z,
        0b10 => Trit::P,
        _ => unreachable!("table contains an invalid 2-bit trit"),
    }
}

fn digit_to_trit(digit: u8) -> Trit {
    match digit {
        0 => Trit::N,
        1 => Trit::Z,
        2 => Trit::P,
        _ => unreachable!("base-3 digit must be 0, 1, or 2"),
    }
}

const INVALID: u16 = u16::MAX;

fn encode_1(trits: &[Trit]) -> Vec<u8> {
    trits.iter().copied().map(trit_to_digit).collect()
}

fn encode_4(trits: &[Trit]) -> Vec<u8> {
    trits
        .chunks(4)
        .map(|chunk| {
            let mut byte = 0_u8;
            let mut place = 1_u8;

            for trit in chunk {
                byte += trit_to_digit(*trit) * place;
                place *= 3;
            }

            byte
        })
        .collect()
}

fn encode_5(trits: &[Trit]) -> Vec<u8> {
    trits
        .chunks(5)
        .map(|chunk| {
            let mut byte = 0_u8;
            let mut place = 1_u8;

            for trit in chunk {
                byte += trit_to_digit(*trit) * place;
                place *= 3;
            }

            byte
        })
        .collect()
}

const fn make_decode_table(trits_per_byte: u16, valid_byte_count: u16) -> [u16; 256] {
    let mut table = [INVALID; 256];
    let mut byte = 0_u16;

    while byte < valid_byte_count {
        let mut remaining = byte;
        let mut packed_trits = 0_u16;
        let mut trit_index = 0_u16;

        while trit_index < trits_per_byte {
            let digit = remaining % 3;
            packed_trits |= digit << (trit_index * 2);
            remaining /= 3;
            trit_index += 1;
        }

        table[byte as usize] = packed_trits;
        byte += 1;
    }

    table
}

const DECODE_1_TABLE: [u16; 256] = make_decode_table(1, 3);
const DECODE_4_TABLE: [u16; 256] = make_decode_table(4, 81);
const DECODE_5_TABLE: [u16; 256] = make_decode_table(5, 243);

fn decode_1_table(bytes: &[u8], trit_count: usize) -> Vec<Trit> {
    decode_with_table(bytes, trit_count, 1, &DECODE_1_TABLE)
}

fn decode_4_table(bytes: &[u8], trit_count: usize) -> Vec<Trit> {
    decode_with_table(bytes, trit_count, 4, &DECODE_4_TABLE)
}

fn decode_5_table(bytes: &[u8], trit_count: usize) -> Vec<Trit> {
    decode_with_table(bytes, trit_count, 5, &DECODE_5_TABLE)
}

fn decode_with_table(
    bytes: &[u8],
    trit_count: usize,
    trits_per_byte: usize,
    table: &[u16; 256],
) -> Vec<Trit> {
    let mut result = Vec::with_capacity(trit_count);

    for &byte in bytes {
        let packed = table[usize::from(byte)];

        assert_ne!(packed, INVALID, "invalid packed ternary byte: {byte}");

        for index in 0..trits_per_byte {
            if result.len() == trit_count {
                return result;
            }

            let bits = (packed >> (index * 2)) & 0b11;
            result.push(bits_to_trit(bits));
        }
    }

    result
}

fn decode_1_direct(bytes: &[u8], trit_count: usize) -> Vec<Trit> {
    assert!(
        bytes.len() >= trit_count,
        "not enough bytes for requested trit count",
    );

    bytes[..trit_count]
        .iter()
        .copied()
        .map(digit_to_trit)
        .collect()
}

fn decode_4_div(bytes: &[u8], trit_count: usize) -> Vec<Trit> {
    let mut result = Vec::with_capacity(trit_count);

    for &byte in bytes {
        let mut value = byte;

        for _ in 0..4 {
            if result.len() == trit_count {
                return result;
            }

            result.push(digit_to_trit(value % 3));
            value /= 3;
        }
    }

    result
}

fn decode_5_div(bytes: &[u8], trit_count: usize) -> Vec<Trit> {
    let mut result = Vec::with_capacity(trit_count);

    for &byte in bytes {
        let mut value = byte;

        for _ in 0..5 {
            if result.len() == trit_count {
                return result;
            }

            result.push(digit_to_trit(value % 3));
            value /= 3;
        }
    }

    result
}

fn assert_tables_match_division() {
    for byte in 0_u8..81 {
        assert_eq!(
            decode_4_table(&[byte], 4),
            decode_4_div(&[byte], 4),
            "4-trit table differs for byte {byte}",
        );
    }

    for byte in 81_u8..=u8::MAX {
        assert_eq!(
            DECODE_4_TABLE[usize::from(byte)],
            INVALID,
            "invalid 4-trit byte {byte} has a table entry",
        );
    }

    for byte in 0_u8..243 {
        assert_eq!(
            decode_5_table(&[byte], 5),
            decode_5_div(&[byte], 5),
            "5-trit table differs for byte {byte}",
        );
    }

    for byte in 243_u8..=u8::MAX {
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
        assert_eq!(decode_1_table(&encoded_1, length), trits);
        assert_eq!(decode_4_div(&encoded_4, length), trits);
        assert_eq!(decode_4_table(&encoded_4, length), trits);
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

fn benchmark_layouts(c: &mut Criterion) {
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
            BenchmarkId::new("1/decode-table", length),
            &encoded_1,
            |b, bytes| b.iter(|| decode_1_table(black_box(bytes), length)),
        );

        _ = group.bench_with_input(
            BenchmarkId::new("4/decode-div", length),
            &encoded_4,
            |b, bytes| b.iter(|| decode_4_div(black_box(bytes), length)),
        );

        _ = group.bench_with_input(
            BenchmarkId::new("4/decode-table", length),
            &encoded_4,
            |b, bytes| b.iter(|| decode_4_table(black_box(bytes), length)),
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

criterion_group!(benches, benchmark_layouts);
criterion_main!(benches);
