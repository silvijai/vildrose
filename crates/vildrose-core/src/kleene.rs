//! Kleene logic for ternary computation

use crate::trit::Trit;

/// A struct representing a Kleene logic value, which can be one of three states: True (T), False (F), or Unknown (U).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Kleene {
    t: Trit,
}

impl Kleene {
    /// Constant representing the False value in Kleene logic.
    pub const FALSE: Self = Self { t: Trit::N };

    /// Constant representing the Unknown value in Kleene logic.
    pub const UNKNOWN: Self = Self { t: Trit::Z };

    /// Constant representing the True value in Kleene logic.
    pub const TRUE: Self = Self { t: Trit::P };

    /// Creates a new Kleene value from a Trit.
    pub const fn new(t: Trit) -> Self {
        Self { t }
    }

    /// Returns the underlying Trit value.
    pub const fn trit(&self) -> Trit {
        self.t
    }

    /// Returns true if the Kleene value is True.
    pub const fn is_true(self) -> bool {
        matches!(self.t, Trit::P)
    }

    /// Returns true if the Kleene value is False.
    pub const fn is_false(self) -> bool {
        matches!(self.t, Trit::N)
    }

    /// Returns true if the Kleene value is Unknown.
    pub const fn is_unknown(self) -> bool {
        matches!(self.t, Trit::Z)
    }

    /// Returns the Kleene AND of two Kleene values.
    #[must_use]
    pub const fn and(self, other: Self) -> Self {
        Self::new(self.t.tmin(other.t))
    }

    /// Returns the Kleene OR of two Kleene values.
    #[must_use]
    pub const fn or(self, other: Self) -> Self {
        Self::new(self.t.tmax(other.t))
    }

    /// Kleene implication: ¬a ∨ b (material implication, Kleene semantics).
    #[must_use]
    pub fn implies(self, other: Self) -> Self {
        (!self).or(other)
    }

    /// Kleene biconditional (equivalence): (a → b) ∧ (b → a).
    #[must_use]
    pub fn iff(self, other: Self) -> Self {
        self.implies(other).and(other.implies(self))
    }
}

impl std::ops::Not for Kleene {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::new(self.t.tnot())
    }
}
