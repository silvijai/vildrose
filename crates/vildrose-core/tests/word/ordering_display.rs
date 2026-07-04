// Tests ensuring that the ordering and display of Trytes is consistent with their underlying Trit representation.

use vildrose_core::{
    trit::Trit,
    word::{Tryte, Word27, Word54},
};

#[test]
fn tryte_display() {
    let t = Tryte::new([
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::Z,
        Trit::Z,
        Trit::Z,
    ]);
    assert_eq!(t.to_string(), "ZZZPZNPZN");
}

#[test]
fn word27_display() {
    let mut trits = [Trit::Z; 27];
    trits[0] = Trit::N;
    trits[1] = Trit::P;
    let w = Word27::new(trits);
    let display = w.to_string();
    assert_eq!(display.len(), 27);
    assert_eq!(display, "ZZZZZZZZZZZZZZZZZZZZZZZZZPN");
}

#[test]
fn word54_display() {
    let mut trits = [Trit::Z; 54];
    trits[0] = Trit::P;
    trits[1] = Trit::N;
    let w = Word54::new(trits);
    let display = w.to_string();
    assert_eq!(display.len(), 54);
    assert_eq!(
        display,
        "ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZNP"
    );
}
