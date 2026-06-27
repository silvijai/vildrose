//! A single balanced ternary property. Analogous to a bit in binary, just with three states: N, Z and P.
use std::fmt::Write;

/// A single balanced ternary property. Analogous to a bit in binary, just with three states: N, Z and P.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i8)]
pub enum Trit {
    /// Trit value referring to, negative, -1 or unknown
    N = -1,
    /// Trit value referring to, zero, 0, blank or false
    Z = 0,
    /// Trit value referring to, positive, 1 or true
    P = 1,
}

/// Helper function for indexes for the static tables.
const fn idx(t: Trit) -> usize {
    (t as i8 + 1).cast_unsigned() as usize
}

/// Tritwise minimum (follows Kleene AND logic).
///
/// ```text
///     N  Z  P
///  N [N, N, N]
///  Z [N, Z, Z]
///  P [N, Z, P]
/// ```
static TMIN: [Trit; 9] = {
    use Trit::{N, P, Z};
    [N, N, N, N, Z, Z, N, Z, P]
};

/// Tritwise maximum (follows Kleene OR logic).
///
/// ```text
///     N  Z  P
///  N [N, Z, P]
///  Z [Z, Z, P]
///  P [P, P, P]
/// ```
static TMAX: [Trit; 9] = {
    use Trit::{N, P, Z};
    [N, Z, P, Z, Z, P, P, P, P]
};

/// Sum trit from single-trit addition (without carry-in).
///
/// ```text
///      N  Z  P
///  N [ P, N, Z]   (-1)+(-1)=-2 → trit P carry N
///  Z [ N, Z, P]   (-1)+0  =-1 → trit N carry Z
///  P [ Z, P, N]   (-1)+1  = 0 → trit Z carry Z
/// ```
static ADD_SUM: [Trit; 9] = {
    use Trit::{N, P, Z};
    [P, N, Z, N, Z, P, Z, P, N]
};

/// Carry trit from single-trit addition (without carry-in).
///
/// Only N+N produces a negative carry, only P+P a positive carry.
///
/// ```text
///     N  Z  P
///  N [N, Z, Z]
///  Z [Z, Z, Z]
///  P [Z, Z, P]
/// ```
static ADD_CARRY: [Trit; 9] = {
    use Trit::{N, P, Z};
    [N, Z, Z, Z, Z, Z, Z, Z, P]
};

/// Consensus function (majority voting).
///
/// Returns Z if either input is Z, otherwise returns the agreed-upon value if both match,
/// or N if they disagree (uncertain/false outcome).
///
/// ```text
///     N  Z  P
///  N [N, Z, N]  consensus(N, N) = N, (N, Z) = Z, (N, P) = N (opposite→N)
///  Z [Z, Z, Z]  consensus with Z is always Z (absorbing)
///  P [N, Z, P]  consensus(P, N) = N (opposite→N), (P, Z) = Z, (P, P) = P
/// ```
static CONSENSUS: [Trit; 9] = {
    use Trit::{N, P, Z};
    [N, Z, N, Z, Z, Z, N, Z, P]
};

// <- Implementation logic starts here
impl Trit {
    /// Construct a new trit using an i8
    ///
    /// For untrusted input, use [`TryFrom<i8>`].
    // const is used here for better caching later
    pub const fn new(val: i8) -> Self {
        match val {
            -1 => Self::N,
            1 => Self::P,
            _ => Self::Z,
        }
    }

    /// Return value as i8 for a trit
    pub const fn value(self) -> i8 {
        self as i8
    }

    /// Return the opposite (negated) for a trit
    #[must_use]
    pub const fn negate(self) -> Self {
        match self {
            Self::N => Self::P,
            Self::Z => Self::Z,
            Self::P => Self::N,
        }
    }

    /// Return the absolute value (no negatives) for a trit
    #[must_use]
    pub const fn abs(self) -> Self {
        match self {
            Self::N => Self::P,
            other => other,
        }
    }

    /// Return the sign of a trit (returns itself)
    ///
    /// It's implemented here, for compatibility with Word27 and other types
    #[must_use]
    pub const fn sign(self) -> Self {
        self
    }

    /// Return whether a trit is zero (0)
    pub const fn is_zero(self) -> bool {
        matches!(self, Self::Z)
    }

    /// Return whether a trit is positive (+1)
    pub const fn is_positive(self) -> bool {
        matches!(self, Self::P)
    }

    /// Return whether a trit is negative (-1)
    pub const fn is_negative(self) -> bool {
        matches!(self, Self::N)
    }

    /// Tritwise minimum (follows Kleene AND logic).
    #[must_use]
    #[inline]
    pub const fn tmin(self, other: Self) -> Self {
        TMIN[idx(self) * 3 + idx(other)]
    }

    /// Tritwise maximum (follows Kleene OR logic).
    #[must_use]
    #[inline]
    pub const fn tmax(self, other: Self) -> Self {
        TMAX[idx(self) * 3 + idx(other)]
    }

    /// Returns the sign of a trit (returns itself)
    #[must_use]
    #[inline]
    pub const fn clip(self) -> Self {
        self
    }

    /// Single-trit addition. Returns (sum, carry).
    ///
    /// The carry must be propagated by the caller into the next trit position.
    #[must_use]
    #[inline]
    pub const fn add(self, other: Self) -> (Self, Self) {
        let i = idx(self) * 3 + idx(other);
        (ADD_SUM[i], ADD_CARRY[i])
    }

    /// Consensus: Z if either is Z, P if equal, N if opposite.
    #[must_use]
    #[inline]
    pub const fn consensus(self, other: Self) -> Self {
        CONSENSUS[idx(self) * 3 + idx(other)]
    }
}

impl From<Trit> for i8 {
    #[inline]
    fn from(t: Trit) -> Self {
        t as Self
    }
}

impl TryFrom<i8> for Trit {
    type Error = &'static str;

    fn try_from(val: i8) -> Result<Self, Self::Error> {
        match val {
            -1 => Ok(Self::N),
            0 => Ok(Self::Z),
            1 => Ok(Self::P),
            _ => Err("Trit value must be -1, 0, or 1."),
        }
    }
}

impl std::fmt::Display for Trit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::N => 'N',
            Self::Z => 'Z',
            Self::P => 'P',
        })
    }
}

// <- Tests start here
#[cfg(test)]
mod tests {
    use super::*;

    // const ratchet
    const _: Trit = Trit::N.negate();
    const _: Trit = Trit::P.tmin(Trit::Z);
    const _: Trit = Trit::N.tmax(Trit::P);
    const _: (Trit, Trit) = Trit::P.add(Trit::P);
    const _: () = assert!(Trit::N.negate() as i8 == 1);
    const _: () = assert!(Trit::P.add(Trit::P).1 as i8 == 1);

    // negate
    #[test]
    fn negate_is_involution() {
        for t in [Trit::N, Trit::Z, Trit::P] {
            assert_eq!(t.negate().negate(), t);
        }
    }

    // tmin / tmax
    #[test]
    fn tmin_tmax_are_symmetric() {
        let trits = [Trit::N, Trit::Z, Trit::P];
        for a in trits {
            for b in trits {
                assert_eq!(a.tmin(b), b.tmin(a), "tmin not symmetric: {a} {b}");
                assert_eq!(a.tmax(b), b.tmax(a), "tmax not symmetric: {a} {b}");
            }
        }
    }

    // clip
    const _: Trit = Trit::N.clip();
    const _: () = assert!(Trit::P.clip() as i8 == 1);
    const _: () = assert!(Trit::N.clip() as i8 == -1);
    const _: () = assert!(Trit::Z.clip() as i8 == 0);

    // add
    #[test]
    fn add_exhaustive() {
        type Case = ((Trit, Trit), (Trit, Trit));
        let cases: &[Case] = &[
            ((Trit::N, Trit::N), (Trit::P, Trit::N)),
            ((Trit::N, Trit::Z), (Trit::N, Trit::Z)),
            ((Trit::N, Trit::P), (Trit::Z, Trit::Z)),
            ((Trit::Z, Trit::N), (Trit::N, Trit::Z)),
            ((Trit::Z, Trit::Z), (Trit::Z, Trit::Z)),
            ((Trit::Z, Trit::P), (Trit::P, Trit::Z)),
            ((Trit::P, Trit::N), (Trit::Z, Trit::Z)),
            ((Trit::P, Trit::Z), (Trit::P, Trit::Z)),
            ((Trit::P, Trit::P), (Trit::N, Trit::P)),
        ];
        for &((a, b), expected) in cases {
            assert_eq!(a.add(b), expected, "add({a}, {b})");
        }
    }

    // display
    #[test]
    fn display_nzp() {
        assert_eq!(Trit::N.to_string(), "N");
        assert_eq!(Trit::Z.to_string(), "Z");
        assert_eq!(Trit::P.to_string(), "P");
    }

    // conversion from i8
    #[test]
    fn try_from_i8_valid() {
        assert_eq!(Trit::try_from(-1i8), Ok(Trit::N));
        assert_eq!(Trit::try_from(0i8), Ok(Trit::Z));
        assert_eq!(Trit::try_from(1i8), Ok(Trit::P));
    }

    #[test]
    fn try_from_i8_invalid() {
        assert!(Trit::try_from(2i8).is_err());
        assert!(Trit::try_from(-2i8).is_err());
        assert!(Trit::try_from(127i8).is_err());
    }

    #[test]
    fn from_i8_coercion() {
        assert_eq!(i8::from(Trit::N), -1);
        assert_eq!(i8::from(Trit::Z), 0);
        assert_eq!(i8::from(Trit::P), 1);
    }

    // tmin / tmax properties
    #[test]
    fn tmin_identity() {
        for t in [Trit::N, Trit::Z, Trit::P] {
            assert_eq!(t.tmin(t), t, "tmin(t, t) should equal t");
        }
    }

    #[test]
    fn tmax_identity() {
        for t in [Trit::N, Trit::Z, Trit::P] {
            assert_eq!(t.tmax(t), t, "tmax(t, t) should equal t");
        }
    }

    #[test]
    fn tmin_absorbing_n() {
        // N is absorbing element for tmin (Kleene AND)
        let n = Trit::N;
        for t in [Trit::N, Trit::Z, Trit::P] {
            assert_eq!(n.tmin(t), Trit::N, "N ∧ x should be N");
            assert_eq!(t.tmin(n), Trit::N, "x ∧ N should be N");
        }
    }

    #[test]
    fn tmax_absorbing_p() {
        // P is absorbing element for tmax (Kleene OR)
        let p = Trit::P;
        for t in [Trit::N, Trit::Z, Trit::P] {
            assert_eq!(p.tmax(t), Trit::P, "P ∨ x should be P");
            assert_eq!(t.tmax(p), Trit::P, "x ∨ P should be P");
        }
    }

    // add properties
    #[test]
    fn add_zero_identity() {
        for t in [Trit::N, Trit::Z, Trit::P] {
            let (sum, carry) = t.add(Trit::Z);
            assert_eq!(sum, t, "t + 0 sum should be t");
            assert_eq!(carry, Trit::Z, "t + 0 carry should be 0");
        }
    }

    #[test]
    fn add_commutative() {
        let trits = [Trit::N, Trit::Z, Trit::P];
        for a in trits {
            for b in trits {
                assert_eq!(
                    a.add(b),
                    b.add(a),
                    "add({a}, {b}) should equal add({b}, {a})"
                );
            }
        }
    }

    #[test]
    fn add_n_n_produces_carry() {
        let (sum, carry) = Trit::N.add(Trit::N);
        assert_eq!(
            sum,
            Trit::P,
            "(-1) + (-1) sum should be 1 (wraps to 1 from -2)"
        );
        assert_eq!(carry, Trit::N, "(-1) + (-1) carry should be -1");
    }

    #[test]
    fn add_p_p_produces_carry() {
        let (sum, carry) = Trit::P.add(Trit::P);
        assert_eq!(sum, Trit::N, "1 + 1 sum should be -1 (wraps to -1 from 2)");
        assert_eq!(carry, Trit::P, "1 + 1 carry should be 1");
    }

    // consensus
    #[test]
    fn consensus_with_z_is_z() {
        let z = Trit::Z;
        for t in [Trit::N, Trit::Z, Trit::P] {
            assert_eq!(z.consensus(t), Trit::Z, "consensus(Z, x) should be Z");
            assert_eq!(t.consensus(z), Trit::Z, "consensus(x, Z) should be Z");
        }
    }

    #[test]
    fn consensus_same_value() {
        for t in [Trit::N, Trit::Z, Trit::P] {
            assert_eq!(t.consensus(t), t, "consensus(t, t) should be t");
        }
    }

    #[test]
    fn consensus_opposite() {
        assert_eq!(
            Trit::N.consensus(Trit::P),
            Trit::N,
            "consensus(N, P) should be N"
        );
        assert_eq!(
            Trit::P.consensus(Trit::N),
            Trit::N,
            "consensus(P, N) should be N"
        );
    }

    // abs
    #[test]
    fn abs_is_idempotent() {
        for t in [Trit::N, Trit::Z, Trit::P] {
            assert_eq!(t.abs().abs(), t.abs(), "abs(abs(t)) == abs(t)");
        }
    }

    #[test]
    fn abs_positive_unchanged() {
        assert_eq!(Trit::Z.abs(), Trit::Z);
        assert_eq!(Trit::P.abs(), Trit::P);
    }

    #[test]
    fn abs_negative_becomes_positive() {
        assert_eq!(Trit::N.abs(), Trit::P);
    }

    // Predicates
    #[test]
    fn is_zero() {
        assert!(Trit::Z.is_zero());
        assert!(!Trit::N.is_zero());
        assert!(!Trit::P.is_zero());
    }

    #[test]
    fn is_positive() {
        assert!(Trit::P.is_positive());
        assert!(!Trit::N.is_positive());
        assert!(!Trit::Z.is_positive());
    }

    #[test]
    fn is_negative() {
        assert!(Trit::N.is_negative());
        assert!(!Trit::P.is_negative());
        assert!(!Trit::Z.is_negative());
    }

    // clip (identity operation)
    #[test]
    fn clip_is_identity() {
        for t in [Trit::N, Trit::Z, Trit::P] {
            assert_eq!(t.clip(), t, "clip should be identity");
        }
    }

    // sign (identity for trits)
    #[test]
    fn sign_is_identity() {
        for t in [Trit::N, Trit::Z, Trit::P] {
            assert_eq!(t.sign(), t, "sign should be identity for trits");
        }
    }
}
