use crate::common::{tryte_p0, word27_p1, word54_p2};
use vildrose_core::trit::Trit;
use vildrose_core::word::{Tryte, Word27, Word54};

#[test]
fn tryte_negate() {
    let t = Tryte::new([
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ]);
    let neg = t.negate();
    assert_eq!(neg.trit(0), Trit::P);
    assert_eq!(neg.trit(1), Trit::Z);
    assert_eq!(neg.trit(2), Trit::N);
}

#[test]
fn tryte_negate_involution() {
    let t = Tryte::new([
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ]);
    assert_eq!(t.negate().negate(), t);
}

#[test]
fn tryte_abs_positive() {
    let t = Tryte::new([
        Trit::P,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ]);
    assert_eq!(t.abs(), t);
}

#[test]
fn tryte_abs_negative() {
    let t = Tryte::new([
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::N,
    ]);
    assert_eq!(t.abs().trit(8), Trit::P);
}

#[test]
fn tryte_sign_positive() {
    let t = Tryte::new([
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::P,
    ]);
    assert_eq!(t.sign(), Trit::P);
}

#[test]
fn tryte_sign_negative() {
    let t = Tryte::new([
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::N,
    ]);
    assert_eq!(t.sign(), Trit::N);
}

#[test]
fn tryte_sign_zero() {
    assert_eq!(Tryte::zero().sign(), Trit::Z);
}

#[test]
fn word27_sign() {
    let mut trits = [Trit::Z; 27];
    trits[26] = Trit::N;
    assert_eq!(Word27::new(trits).sign(), Trit::N);
}

#[test]
fn word27_negate() {
    let mut trits = [Trit::Z; 27];
    trits[0] = Trit::P;
    let neg = Word27::new(trits).negate();
    assert_eq!(neg.trit(0), Trit::N);
    for i in 1..27 {
        assert_eq!(neg.trit(i), Trit::Z);
    }
}

#[test]
fn tryte_plus_tryte() {
    assert_eq!((tryte_p0() + tryte_p0()).trit(1), Trit::P);
}

#[test]
fn word27_plus_word27() {
    assert_eq!((word27_p1() + word27_p1()).trit(2), Trit::P);
}

#[test]
fn word54_plus_word54() {
    assert_eq!((word54_p2() + word54_p2()).trit(3), Trit::P);
}

#[test]
fn tryte_add_overflow_wraps_to_min() {
    let max = Tryte::new([Trit::P; 9]);
    let sum = max + tryte_p0();
    assert_eq!(sum.sign(), Trit::N);
    assert_eq!(i16::from(sum), Tryte::MIN_INT);
}

#[test]
fn word27_add_overflow_wraps_to_min() {
    let max = Word27::new([Trit::P; 27]);
    let sum = max + tryte_p0();
    assert_eq!(sum.sign(), Trit::N);
    assert_eq!(i64::from(sum), Word27::MIN_INT);
}

#[test]
fn word54_add_overflow_wraps_to_min() {
    let max = Word54::new([Trit::P; 54]);
    let sum = max + tryte_p0();
    assert_eq!(sum.sign(), Trit::N);
    assert_eq!(i128::from(sum), Word54::MIN_INT);
}
