// Trit shift tests done here, focusing on shifting logic.

use vildrose_core::{trit::Trit, word::Tryte};

// <- Tryte trit shifts
#[test]
fn tryte_tshl() {
    let t = Tryte::new([
        Trit::P,
        Trit::N,
        Trit::P,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ]);
    let shifted = t.tshl(1);
    assert_eq!(shifted.trit(0), Trit::Z);
    assert_eq!(shifted.trit(1), Trit::P);
    assert_eq!(shifted.trit(2), Trit::N);
}

#[test]
fn tryte_tshr_positive() {
    let t = Tryte::new([
        Trit::N,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::P,
    ]);
    let shifted = t.tshr(1);
    assert_eq!(shifted.trit(0), Trit::Z);
    assert_eq!(shifted.trit(7), Trit::P);
    assert_eq!(shifted.trit(8), Trit::P);
}

#[test]
fn tryte_tshr_negative() {
    let t = Tryte::new([
        Trit::P,
        Trit::N,
        Trit::P,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::N,
    ]);
    let shifted = t.tshr(1);
    assert_eq!(shifted.trit(8), Trit::N);
}

#[test]
fn tryte_tlshr() {
    let t = Tryte::new([
        Trit::P,
        Trit::N,
        Trit::P,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ]);
    let shifted = t.tlshr(1);
    assert_eq!(shifted.trit(0), Trit::N);
    assert_eq!(shifted.trit(1), Trit::P);
    assert_eq!(shifted.trit(8), Trit::Z);
}

#[test]
fn tryte_tshl_large_shift() {
    let t = Tryte::new([
        Trit::P,
        Trit::N,
        Trit::P,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ]);
    let shifted = t.tshl(10);
    for i in 0..9 {
        assert_eq!(shifted.trit(i), Trit::Z);
    }
}

#[test]
fn tryte_tshr_large_shift_fills_with_positive_sign() {
    let value = Tryte::from_int(1).unwrap();

    let shifted = value.tshr(Tryte::TRIT_COUNT);

    assert_eq!(shifted, Tryte::new([Trit::P; Tryte::TRIT_COUNT]));
}

#[test]
fn tryte_tshr_large_shift_fills_with_negative_sign() {
    let value = Tryte::from_int(-1).unwrap();

    let shifted = value.tshr(Tryte::TRIT_COUNT);

    assert_eq!(shifted, Tryte::new([Trit::N; Tryte::TRIT_COUNT]));
}

#[test]
fn tryte_tshr_large_shift_of_zero_is_zero() {
    assert_eq!(Tryte::zero().tshr(Tryte::TRIT_COUNT), Tryte::zero());
}

#[test]
fn tryte_tlshr_large_shift() {
    let t = Tryte::new([
        Trit::P,
        Trit::N,
        Trit::P,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ]);
    let shifted = t.tlshr(10);
    for i in 0..9 {
        assert_eq!(shifted.trit(i), Trit::Z);
    }
}
