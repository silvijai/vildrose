use super::Word;
use crate::trit::Trit;
use core::cmp::Ordering;
use core::fmt;
use core::ops::Neg;

// TODO: Look into how to compact the words by storing 4 trits per byte

impl<const N: usize> Word<N> {
    /// Number of trits in this word.
    pub const TRIT_COUNT: usize = N;

    /// Creates a word from little-endian trits.
    ///
    /// The first array element is the least-significant trit.
    #[must_use]
    pub const fn new(trits: [Trit; N]) -> Self {
        Self(trits)
    }

    /// Creates a word with all trits equal to zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self([Trit::Z; N])
    }

    /// Returns the stored trits in little-endian order.
    #[must_use]
    pub const fn into_trits(self) -> [Trit; N] {
        self.0
    }

    /// Returns the trit at `index`, or `None` if it is out of bounds.
    ///
    /// Trit zero is the least-significant trit.
    #[must_use]
    pub fn get_trit(self, index: usize) -> Option<Trit> {
        self.0.get(index).copied()
    }

    /// Returns the trit at `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index >= Self::TRIT_COUNT`.
    #[must_use]
    pub const fn trit(self, index: usize) -> Trit {
        self.0[index]
    }

    /// Returns the arithmetic negation of this word.
    #[must_use]
    pub fn negate(self) -> Self {
        Self(self.0.map(Trit::negate))
    }

    /// Returns `Trit::N`, `Trit::Z`, or `Trit::P` according to numeric sign.
    #[must_use]
    pub fn sign(self) -> Trit {
        self.0
            .iter()
            .rev()
            .copied()
            .find(|&trit| trit != Trit::Z)
            .unwrap_or(Trit::Z)
    }

    /// Returns the non-negative magnitude of this word.
    #[must_use]
    pub fn abs(self) -> Self {
        if self.sign() == Trit::N { -self } else { self }
    }

    /// Truncates this word to its least-significant `W` trits.
    ///
    /// # Panics
    ///
    /// Panics if `W > N`. This is a raw fixed-width truncation operation;
    /// it does not check whether the value can be represented losslessly.
    #[must_use]
    pub fn truncate<const W: usize>(self) -> Word<W> {
        assert!(W <= N, "cannot truncate Word<{N}> into wider Word<{W}>");

        Word(core::array::from_fn(|index| self.0[index]))
    }
}

impl<const N: usize> Neg for Word<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl<const N: usize> PartialOrd for Word<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for Word<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .iter()
            .rev()
            .zip(other.0.iter().rev())
            .map(|(left, right)| left.cmp(right))
            .find(|&ordering| ordering != Ordering::Equal)
            .unwrap_or(Ordering::Equal)
    }
}

impl<const N: usize> fmt::Display for Word<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for trit in self.0.iter().rev() {
            write!(f, "{trit}")?;
        }

        Ok(())
    }
}
