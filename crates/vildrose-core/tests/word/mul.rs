use vildrose_core::word::{Tryte, Word27, Word54};

#[test]
fn tryte_mul_by_zero_is_zero() {
    let t = Tryte::from_int(123).unwrap();
    let zero = Tryte::zero();
    assert_eq!((t * zero).to_int(), 0);
}

#[test]
fn tryte_mul_identity() {
    let t = Tryte::from_int(123).unwrap();
    let one = Tryte::from_int(1).unwrap();
    assert_eq!(t * one, t);
}

#[test]
fn tryte_mul_basic() {
    let a = Tryte::from_int(3).unwrap();
    let b = Tryte::from_int(4).unwrap();
    assert_eq!((a * b).to_int(), 12);
}

#[test]
fn tryte_mul_negative_operand() {
    let a = Tryte::from_int(-3).unwrap();
    let b = Tryte::from_int(4).unwrap();
    assert_eq!((a * b).to_int(), -12);
}

#[test]
fn tryte_mul_two_negatives() {
    let a = Tryte::from_int(-3).unwrap();
    let b = Tryte::from_int(-4).unwrap();
    assert_eq!((a * b).to_int(), 12);
}

#[test]
fn tryte_mul_commutative() {
    let a = Tryte::from_int(17).unwrap();
    let b = Tryte::from_int(-6).unwrap();
    assert_eq!(a * b, b * a);
}

#[test]
fn tryte_mul_overflow_wraps() {
    // 9841 * 2 = 19682, which is one short of the full range (3^9 = 19683),
    // so it wraps to -1 in the centered representation.
    let max = Tryte::from_int(Tryte::MAX_INT).unwrap();
    let two = Tryte::from_int(2).unwrap();
    assert_eq!((max * two).to_int(), -1);
}

#[test]
fn word27_mul_basic() {
    let a = Word27::from_int(100).unwrap();
    let b = Word27::from_int(200).unwrap();
    assert_eq!((a * b).to_int(), 20_000);
}

#[test]
fn tryte_mul_word27_cross_width() {
    let t = Tryte::from_int(3).unwrap();
    let w = Word27::from_int(4).unwrap();
    let product = t * w;
    assert_eq!(product.to_int(), 12);
}

#[test]
fn word27_mul_tryte_cross_width_commutes() {
    let t = Tryte::from_int(3).unwrap();
    let w = Word27::from_int(4).unwrap();
    assert_eq!((t * w).to_int(), (w * t).to_int());
}

#[test]
fn word54_mul_tryte_cross_width() {
    let t = Tryte::from_int(5).unwrap();
    let w = Word54::from_int(7).unwrap();
    assert_eq!((t * w).to_int(), 35);
}

#[test]
fn word54_mul_word27_cross_width() {
    let a = Word27::from_int(9).unwrap();
    let b = Word54::from_int(11).unwrap();
    assert_eq!((a * b).to_int(), 99);
}
