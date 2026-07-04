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
/// Returns Z if the inputs disagree or if either input is Z.
/// Returns the shared value only if both inputs agree.
///
/// ```text
///        N    Z    P
/// N  [   N    Z    Z  ]  consensus(N,N) = N, consensus(N,Z) = Z, consensus(N,P) = Z
/// Z  [   Z    Z    Z  ]  consensus with Z is always Z (absorbing)
/// P  [   Z    Z    P  ]  consensus(P,N) = Z, consensus(P,Z) = Z, consensus(P,P) = P
/// ```
static CONSENSUS: [Trit; 9] = {
    use Trit::{N, P, Z};
    [N, Z, Z, Z, Z, Z, Z, Z, P]
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

    /// Return the incremented value for a trit, wrapping  P -> N.
    #[must_use]
    pub const fn inc(self) -> Self {
        match self {
            Self::N => Self::Z,
            Self::Z => Self::P,
            Self::P => Self::N,
        }
    }

    /// Return the decremented value for a trit, wrapping N -> P.
    #[must_use]
    pub const fn dec(self) -> Self {
        match self {
            Self::N => Self::P,
            Self::Z => Self::N,
            Self::P => Self::Z,
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

    // clip
    const _: Trit = Trit::N.clip();
    const _: () = assert!(Trit::P.clip() as i8 == 1);
    const _: () = assert!(Trit::N.clip() as i8 == -1);
    const _: () = assert!(Trit::Z.clip() as i8 == 0);
}
