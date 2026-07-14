use super::Word;
use crate::trit::Trit;

impl<const N: usize> Word<N> {
    /// Returns the component-wise ternary minimum.
    #[must_use]
    pub fn tmin(self, rhs: Self) -> Self {
        Self(core::array::from_fn(|index| {
            self.0[index].tmin(rhs.0[index])
        }))
    }

    /// Returns the component-wise ternary maximum.
    #[must_use]
    pub fn tmax(self, rhs: Self) -> Self {
        Self(core::array::from_fn(|index| {
            self.0[index].tmax(rhs.0[index])
        }))
    }

    /// Returns the tritwise ternary negation.
    ///
    /// Each `N` becomes `P`, each `P` becomes `N`, and `Z` remains `Z`.
    #[must_use]
    pub fn tnot(self) -> Self {
        Self(self.0.map(Trit::negate))
    }

    /// Returns the tritwise clipping of this word.
    #[must_use]
    pub fn tclip(self) -> Self {
        Self(self.0.map(Trit::clip))
    }

    /// Returns a word-valued numeric sign: -1, 0, or +1.
    #[must_use]
    pub fn signum(self) -> Self {
        Self(core::array::from_fn(|index| {
            if index == 0 { self.sign() } else { Trit::Z }
        }))
    }

    /// Returns the tritwise consensus of two words.
    #[must_use]
    pub fn tconsensus(self, rhs: Self) -> Self {
        Self(core::array::from_fn(|index| {
            self.0[index].consensus(rhs.0[index])
        }))
    }

    /// Shifts trits left by `count`, filling low trits with zero.
    ///
    /// If `count >= Self::TRIT_COUNT`, returns zero.
    #[must_use]
    pub fn tshl(self, count: usize) -> Self {
        Self(core::array::from_fn(|index| {
            index
                .checked_sub(count)
                .and_then(|source| self.0.get(source).copied())
                .unwrap_or(Trit::Z)
        }))
    }

    /// Returns the arithmetic right shift by `count` trits.
    ///
    /// Vacated most-significant trits are filled with the numeric sign.
    /// If `count >= Self::TRIT_COUNT`, every trit becomes the sign trit.
    #[must_use]
    pub fn tshr(self, count: usize) -> Self {
        let sign = self.sign();

        Self(core::array::from_fn(|index| {
            self.0
                .get(index.saturating_add(count))
                .copied()
                .unwrap_or(sign)
        }))
    }

    /// Returns the logical right shift by `count` trits.
    ///
    /// Vacated most-significant trits are filled with zero.
    /// If `count >= Self::TRIT_COUNT`, returns zero.
    #[must_use]
    pub fn tlshr(self, count: usize) -> Self {
        Self(core::array::from_fn(|index| {
            self.0
                .get(index.saturating_add(count))
                .copied()
                .unwrap_or(Trit::Z)
        }))
    }
}
