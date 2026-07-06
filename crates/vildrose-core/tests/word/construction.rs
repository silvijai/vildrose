// Construction related tests for Tryte, Word27, and Word54.
// Test for functions like new, zero, from_trits, etc. that are related to constructing these types.

use vildrose_core::{
    trit::Trit,
    word::{Tryte, Word9, Word27, Word54},
};

// <- Tryte construction tests
#[test]
fn tryte_new_initializes_with_provided_array() {
    let trits = [
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ];
    let t = Tryte::new(trits);
    assert_eq!(t.trit(0), Trit::N);
    assert_eq!(t.trit(1), Trit::Z);
    assert_eq!(t.trit(2), Trit::P);
    for i in 3..9 {
        assert_eq!(t.trit(i), Trit::Z);
    }
}

#[test]
fn tryte_zero() {
    let t = Tryte::zero();
    for i in 0..9 {
        assert_eq!(t.trit(i), Trit::Z);
    }
}

#[test]
fn tryte_from_trits() {
    let trits = [Trit::P; 9];
    let t = Tryte::from_trits(trits);
    for i in 0..9 {
        assert_eq!(t.trit(i), Trit::P);
    }
}

#[test]
fn tryte_trit_access() {
    let trits = [
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
    ];
    let t = Tryte::new(trits);
    for (i, trit) in trits.iter().enumerate() {
        assert_eq!(t.trit(i), *trit);
    }
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn tryte_trit_out_of_bounds() {
    let t = Tryte::zero();
    let _ = t.trit(9);
}

// <- Word9 construction tests (same as Tryte, but for completeness)
#[test]
fn word9_new_initializes_with_provided_array() {
    let trits = [
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ];
    let w = Word9::new(trits);
    assert_eq!(w.trit(0), Trit::N);
    assert_eq!(w.trit(1), Trit::Z);
    assert_eq!(w.trit(2), Trit::P);
    for i in 3..9 {
        assert_eq!(w.trit(i), Trit::Z);
    }
}

#[test]
fn word9_zero() {
    let w = Word9::zero();
    for i in 0..9 {
        assert_eq!(w.trit(i), Trit::Z);
    }
}

#[test]
fn word9_from_trits() {
    let trits = [Trit::P; 9];
    let w = Word9::from_trits(trits);
    for i in 0..9 {
        assert_eq!(w.trit(i), Trit::P);
    }
}

#[test]
fn word9_trit_access() {
    let trits = [
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
    ];
    let w = Word9::new(trits);
    for (i, trit) in trits.iter().enumerate() {
        assert_eq!(w.trit(i), *trit);
    }
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn word9_trit_out_of_bounds() {
    let w = Word9::zero();
    let _ = w.trit(9);
}

// <- Word27 construction tests
#[test]
fn word27_new_initializes_with_provided_array() {
    let trits = [
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ];
    let w = Word27::new(trits);
    assert_eq!(w.trit(0), Trit::N);
    assert_eq!(w.trit(1), Trit::Z);
    assert_eq!(w.trit(2), Trit::P);
    for i in 3..27 {
        assert_eq!(w.trit(i), Trit::Z);
    }
}

#[test]
fn word27_zero() {
    let w = Word27::zero();
    for i in 0..27 {
        assert_eq!(w.trit(i), Trit::Z);
    }
}

#[test]
fn word27_from_trits() {
    let trits = [Trit::P; 27];
    let w = Word27::from_trits(trits);
    for i in 0..27 {
        assert_eq!(w.trit(i), Trit::P);
    }
}

#[test]
fn word27_trit_access() {
    let trits = [
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
    ];
    let w = Word27::new(trits);
    for (i, trit) in trits.iter().enumerate() {
        assert_eq!(w.trit(i), *trit);
    }
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn word27_trit_out_of_bounds() {
    let w = Word27::zero();
    let _ = w.trit(27);
}

// <- Word54 construction tests
#[test]
fn word54_new_initializes_with_provided_array() {
    let trits = [
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ];
    let w = Word54::new(trits);
    assert_eq!(w.trit(0), Trit::N);
    assert_eq!(w.trit(1), Trit::Z);
    assert_eq!(w.trit(2), Trit::P);
    for i in 3..54 {
        assert_eq!(w.trit(i), Trit::Z);
    }
}

#[test]
fn word54_zero() {
    let w = Word54::zero();
    for i in 0..54 {
        assert_eq!(w.trit(i), Trit::Z);
    }
}

#[test]
fn word54_from_trits() {
    let trits = [Trit::P; 54];
    let w = Word54::from_trits(trits);
    for i in 0..54 {
        assert_eq!(w.trit(i), Trit::P);
    }
}

// <- Compile-time checks guaranteeing functions are truly const
const _: Tryte = Tryte::zero();
const _: Tryte = Tryte::from_trits([Trit::Z; 9]);

const _: Word9 = Word9::zero();
const _: Word9 = Word9::from_trits([Trit::Z; 9]);

const _: Word27 = Word27::zero();
const _: Word27 = Word27::from_trits([Trit::Z; 27]);

const _: Word54 = Word54::zero();
const _: Word54 = Word54::from_trits([Trit::Z; 54]);
