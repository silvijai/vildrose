//! Common attributes shared across multiple test modules.

use vildrose_core::trit::Trit;
use vildrose_core::word::{Tryte, Word27, Word54};

/// Returns a Tryte with the first trit set to P and the rest set to Z.
#[allow(dead_code)]
pub const fn tryte_p0() -> Tryte {
    let mut trits = [Trit::Z; 9];
    trits[0] = Trit::P;
    Tryte::new(trits)
}

/// Returns a Word27 with the second trit set to P and the rest set to Z.
#[allow(dead_code)]
pub const fn word27_p1() -> Word27 {
    let mut trits = [Trit::Z; 27];
    trits[1] = Trit::P;
    Word27::new(trits)
}

/// Returns a Word54 with the third trit set to P and the rest set to Z.
#[allow(dead_code)]
pub const fn word54_p2() -> Word54 {
    let mut trits = [Trit::Z; 54];
    trits[2] = Trit::P;
    Word54::new(trits)
}
