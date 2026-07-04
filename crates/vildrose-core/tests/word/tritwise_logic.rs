// Tritwise logic related tests for Tryte, Word27, and Word54.
// Test for functions like tritwise_and, tritwise_or, tritwise_xor, etc. that perform logical operations on these types.

use vildrose_core::{trit::Trit, word::Tryte};

// <- Tryte tritwise logic tests
#[test]
fn tryte_tmin() {
    let t1 = Tryte::new([
        Trit::N,
        Trit::P,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ]);
    let t2 = Tryte::new([
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ]);
    let min = t1.tmin(t2);
    assert_eq!(min.trit(0), Trit::N);
    assert_eq!(min.trit(1), Trit::N);
}

#[test]
fn tryte_tmax() {
    let t1 = Tryte::new([
        Trit::N,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ]);
    let t2 = Tryte::new([
        Trit::P,
        Trit::N,
        Trit::N,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ]);
    let max = t1.tmax(t2);
    assert_eq!(max.trit(0), Trit::P);
    assert_eq!(max.trit(1), Trit::P);
    assert_eq!(max.trit(2), Trit::N);
    assert_eq!(max.trit(3), Trit::Z);
}

#[test]
fn tryte_tnot() {
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
    let not = t.tnot();
    assert_eq!(not.trit(0), Trit::P);
    assert_eq!(not.trit(1), Trit::Z);
    assert_eq!(not.trit(2), Trit::N);
}

#[test]
fn tryte_tconsensus() {
    let t1 = Tryte::new([
        Trit::N,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ]);
    let t2 = Tryte::new([
        Trit::N,
        Trit::P,
        Trit::P,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ]);
    let cons = t1.tconsensus(t2);
    assert_eq!(cons.trit(0), Trit::N);
    assert_eq!(cons.trit(1), Trit::P);
    assert_eq!(cons.trit(2), Trit::Z);
    assert_eq!(cons.trit(3), Trit::Z);
}
