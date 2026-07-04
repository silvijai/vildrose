//! Arithmetic primitives for balanced ternary words.
use crate::trit::Trit;

fn trit_full_add(a: Trit, b: Trit, carry: Trit) -> (Trit, Trit) {
    match a.value() + b.value() + carry.value() {
        -3 => (Trit::Z, Trit::N),
        -2 => (Trit::P, Trit::N),
        -1 => (Trit::N, Trit::Z),
        0 => (Trit::Z, Trit::Z),
        1 => (Trit::P, Trit::Z),
        2 => (Trit::N, Trit::P),
        3 => (Trit::Z, Trit::P),
        _ => unreachable!(),
    }
}

/// Adds two same-width trit arrays. Overflow carry is discarded (wrapping).
pub fn ripple_add<const N: usize>(a: [Trit; N], b: [Trit; N]) -> [Trit; N] {
    let mut result = [Trit::Z; N];
    let mut carry = Trit::Z;

    for i in 0..N {
        let (s, c) = trit_full_add(a[i], b[i], carry);
        result[i] = s;
        carry = c;
    }

    result
}

/// Widens a narrower trit array into a wider one, zero-filling the high trits.
pub fn widen<const FROM: usize, const TO: usize>(a: [Trit; FROM]) -> [Trit; TO] {
    let mut result = [Trit::Z; TO];
    result[..FROM].copy_from_slice(&a);
    result
}

// <- Tests only for private functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test for all 27 cases of trit_full_add, covering all combinations of three trits.
    fn trit_full_add_cases() {
        assert_eq!(trit_full_add(Trit::N, Trit::N, Trit::N), (Trit::Z, Trit::N)); // -1 + -1 + -1 = -3 => (Z, N)
        assert_eq!(trit_full_add(Trit::N, Trit::N, Trit::Z), (Trit::P, Trit::N));
        assert_eq!(trit_full_add(Trit::N, Trit::N, Trit::P), (Trit::N, Trit::Z));
        assert_eq!(trit_full_add(Trit::N, Trit::Z, Trit::N), (Trit::P, Trit::N));
        assert_eq!(trit_full_add(Trit::N, Trit::Z, Trit::Z), (Trit::N, Trit::Z));
        assert_eq!(trit_full_add(Trit::N, Trit::Z, Trit::P), (Trit::Z, Trit::Z));
        assert_eq!(trit_full_add(Trit::N, Trit::P, Trit::N), (Trit::N, Trit::Z));
        assert_eq!(trit_full_add(Trit::N, Trit::P, Trit::Z), (Trit::Z, Trit::Z));
        assert_eq!(trit_full_add(Trit::N, Trit::P, Trit::P), (Trit::P, Trit::Z));
        assert_eq!(trit_full_add(Trit::Z, Trit::N, Trit::N), (Trit::P, Trit::N));
        assert_eq!(trit_full_add(Trit::Z, Trit::N, Trit::Z), (Trit::N, Trit::Z));
        assert_eq!(trit_full_add(Trit::Z, Trit::N, Trit::P), (Trit::Z, Trit::Z));
        assert_eq!(trit_full_add(Trit::Z, Trit::Z, Trit::N), (Trit::N, Trit::Z));
        assert_eq!(trit_full_add(Trit::Z, Trit::Z, Trit::Z), (Trit::Z, Trit::Z)); // 0 + 0 + 0 = 0 => (Z, Z)
        assert_eq!(trit_full_add(Trit::Z, Trit::Z, Trit::P), (Trit::P, Trit::Z));
        assert_eq!(trit_full_add(Trit::Z, Trit::P, Trit::N), (Trit::Z, Trit::Z));
        assert_eq!(trit_full_add(Trit::Z, Trit::P, Trit::Z), (Trit::P, Trit::Z));
        assert_eq!(trit_full_add(Trit::Z, Trit::P, Trit::P), (Trit::N, Trit::P));
        assert_eq!(trit_full_add(Trit::P, Trit::N, Trit::N), (Trit::N, Trit::Z));
        assert_eq!(trit_full_add(Trit::P, Trit::N, Trit::Z), (Trit::Z, Trit::Z));
        assert_eq!(trit_full_add(Trit::P, Trit::N, Trit::P), (Trit::P, Trit::Z));
        assert_eq!(trit_full_add(Trit::P, Trit::Z, Trit::N), (Trit::Z, Trit::Z));
        assert_eq!(trit_full_add(Trit::P, Trit::Z, Trit::Z), (Trit::P, Trit::Z));
        assert_eq!(trit_full_add(Trit::P, Trit::Z, Trit::P), (Trit::N, Trit::P));
        assert_eq!(trit_full_add(Trit::P, Trit::P, Trit::N), (Trit::P, Trit::Z));
        assert_eq!(trit_full_add(Trit::P, Trit::P, Trit::Z), (Trit::N, Trit::P));
        assert_eq!(trit_full_add(Trit::P, Trit::P, Trit::P), (Trit::Z, Trit::P)); // 1 + 1 + 1 = 3 => (Z, P)
    }
}
