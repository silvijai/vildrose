use vildrose_core::word::{Tryte, Word27};

const SAMPLE_VALUES: [i16; 7] = [-9841, -100, -1, 0, 1, 100, 9841];

fn tryte(n: i16) -> Tryte {
    Tryte::from_int(n).unwrap()
}

#[test]
fn add_is_commutative() {
    for &a in &SAMPLE_VALUES {
        for &b in &SAMPLE_VALUES {
            assert_eq!(tryte(a) + tryte(b), tryte(b) + tryte(a));
        }
    }
}

#[test]
fn mul_is_commutative() {
    for &a in &SAMPLE_VALUES {
        for &b in &SAMPLE_VALUES {
            assert_eq!(tryte(a) * tryte(b), tryte(b) * tryte(a));
        }
    }
}

#[test]
fn add_zero_is_identity() {
    for &a in &SAMPLE_VALUES {
        assert_eq!(tryte(a) + Tryte::zero(), tryte(a));
    }
}

#[test]
fn mul_one_is_identity() {
    let one = tryte(1);
    for &a in &SAMPLE_VALUES {
        assert_eq!(tryte(a) * one, tryte(a));
    }
}

#[test]
fn mul_zero_annihilates() {
    for &a in &SAMPLE_VALUES {
        assert_eq!((tryte(a) * Tryte::zero()).to_int(), 0);
    }
}

#[test]
fn neg_is_involution() {
    for &a in &SAMPLE_VALUES {
        assert_eq!(-(-tryte(a)), tryte(a));
    }
}

#[test]
fn sub_self_is_zero() {
    for &a in &SAMPLE_VALUES {
        assert_eq!((tryte(a) - tryte(a)).to_int(), 0);
    }
}

#[test]
fn zero_minus_self_equals_negate() {
    for &a in &SAMPLE_VALUES {
        assert_eq!(Tryte::zero() - tryte(a), -tryte(a));
    }
}

#[test]
fn ord_matches_native_int_ordering() {
    let mut words: Vec<Tryte> = SAMPLE_VALUES.iter().map(|&n| tryte(n)).collect();
    words.sort();
    let mut ints: Vec<i16> = SAMPLE_VALUES.to_vec();
    ints.sort_unstable();
    let sorted_from_words: Vec<i16> = words.iter().map(|w| w.to_int()).collect();
    assert_eq!(sorted_from_words, ints);
}

#[test]
fn sign_matches_int_sign() {
    for &a in &SAMPLE_VALUES {
        let t = tryte(a);
        let expected_positive = a > 0;
        let expected_negative = a < 0;
        assert_eq!(i16::from(t.sign().value()) > 0, expected_positive);
        assert_eq!(i16::from(t.sign().value()) < 0, expected_negative);
    }
}

#[test]
fn abs_is_never_negative() {
    for &a in &SAMPLE_VALUES {
        assert!(tryte(a).abs().to_int() >= 0);
    }
}

#[test]
fn abs_matches_native_abs_within_range() {
    for &a in &SAMPLE_VALUES {
        // -MIN_INT == MAX_INT here since the range is symmetric, so no overflow edge case.
        assert_eq!(tryte(a).abs().to_int(), a.abs());
    }
}

#[test]
fn cross_width_ord_consistent_with_int() {
    let narrow = Tryte::from_int(5).unwrap();
    let wide = Word27::from_int(5).unwrap();
    assert_eq!(i64::from(narrow.to_int()), wide.to_int());
}
