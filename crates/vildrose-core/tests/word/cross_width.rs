use crate::common::{tryte_p0, word27_p1, word54_p2};
use vildrose_core::trit::Trit;

#[test]
fn tryte_plus_word27() {
    let sum = tryte_p0() + word27_p1();
    assert_eq!(sum.trit(0), Trit::P);
    assert_eq!(sum.trit(1), Trit::P);
}

#[test]
fn word27_plus_tryte() {
    let sum = word27_p1() + tryte_p0();
    assert_eq!(sum.trit(0), Trit::P);
    assert_eq!(sum.trit(1), Trit::P);
}

#[test]
fn tryte_plus_word54() {
    let sum = tryte_p0() + word54_p2();
    assert_eq!(sum.trit(0), Trit::P);
    assert_eq!(sum.trit(2), Trit::P);
}

#[test]
fn word54_plus_tryte() {
    let sum = word54_p2() + tryte_p0();
    assert_eq!(sum.trit(0), Trit::P);
    assert_eq!(sum.trit(2), Trit::P);
}

#[test]
fn word27_plus_word54() {
    let sum = word27_p1() + word54_p2();
    assert_eq!(sum.trit(1), Trit::P);
    assert_eq!(sum.trit(2), Trit::P);
}

#[test]
fn word54_plus_word27() {
    let sum = word54_p2() + word27_p1();
    assert_eq!(sum.trit(1), Trit::P);
    assert_eq!(sum.trit(2), Trit::P);
}
