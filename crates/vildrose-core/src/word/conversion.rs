use super::{Tribble, Tryte, Word27, Word54, WordError};
use crate::trit::Trit;

impl Tribble {
    /// The minimum representable value for a tribble.
    pub const MIN_INT: i8 = -(3_i8.pow(3) - 1) / 2;

    /// The maximum representable value for a tribble.
    pub const MAX_INT: i8 = (3_i8.pow(3) - 1) / 2;

    /// Converts this tribble to its native integer representation.
    #[must_use]
    pub fn to_int(self) -> i8 {
        let mut value = 0_i8;
        let mut place = 1_i8;

        for trit in self.0 {
            value += trit.value() * place;
            place *= 3;
        }

        value
    }

    /// Constructs a tribble from a native integer.
    ///
    /// # Errors
    ///
    /// Returns [`WordError::OutOfRange`] if `value` is outside
    /// [`Tribble::MIN_INT`] through [`Tribble::MAX_INT`].
    pub fn from_int(mut value: i8) -> Result<Self, WordError> {
        if !(Self::MIN_INT..=Self::MAX_INT).contains(&value) {
            return Err(WordError::OutOfRange);
        }

        let mut trits = [Trit::Z; Self::TRIT_COUNT];

        for trit in &mut trits {
            let remainder = value.rem_euclid(3);

            *trit = match remainder {
                0 => Trit::Z,
                1 => Trit::P,
                2 => {
                    value += 1;
                    Trit::N
                }
                _ => unreachable!("remainder modulo three is always 0, 1, or 2"),
            };

            value = value.div_euclid(3);
        }

        Ok(Self(trits))
    }
}

impl From<Tribble> for i8 {
    fn from(word: Tribble) -> Self {
        word.to_int()
    }
}

impl TryFrom<i8> for Tribble {
    type Error = WordError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl Tryte {
    /// The minimum representable value for a tryte.
    pub const MIN_INT: i16 = -(3_i16.pow(9) - 1) / 2;

    /// The maximum representable value for a tryte.
    pub const MAX_INT: i16 = (3_i16.pow(9) - 1) / 2;

    /// Converts this tryte to its native integer representation.
    #[must_use]
    pub fn to_int(self) -> i16 {
        let mut value = 0_i16;
        let mut place = 1_i16;

        for trit in self.0 {
            value += i16::from(trit.value()) * place;
            place *= 3;
        }

        value
    }

    /// Constructs a tryte from a native integer.
    ///
    /// # Errors
    ///
    /// Returns [`WordError::OutOfRange`] if `value` is outside
    /// [`Tryte::MIN_INT`] through [`Tryte::MAX_INT`].
    pub fn from_int(mut value: i16) -> Result<Self, WordError> {
        if !(Self::MIN_INT..=Self::MAX_INT).contains(&value) {
            return Err(WordError::OutOfRange);
        }

        let mut trits = [Trit::Z; Self::TRIT_COUNT];

        for trit in &mut trits {
            let remainder = value.rem_euclid(3);

            *trit = match remainder {
                0 => Trit::Z,
                1 => Trit::P,
                2 => {
                    value += 1;
                    Trit::N
                }
                _ => unreachable!("remainder modulo three is always 0, 1, or 2"),
            };

            value = value.div_euclid(3);
        }

        Ok(Self(trits))
    }
}

impl From<Tryte> for i16 {
    fn from(word: Tryte) -> Self {
        word.to_int()
    }
}

impl TryFrom<i16> for Tryte {
    type Error = WordError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl Word27 {
    /// The minimum representable value for a 27-trit word.
    pub const MIN_INT: i64 = -(3_i64.pow(27) - 1) / 2;

    /// The maximum representable value for a 27-trit word.
    pub const MAX_INT: i64 = (3_i64.pow(27) - 1) / 2;

    /// Converts this 27-trit word to its native integer representation.
    #[must_use]
    pub fn to_int(self) -> i64 {
        let mut value = 0_i64;
        let mut place = 1_i64;

        for trit in self.0 {
            value += i64::from(trit.value()) * place;
            place *= 3;
        }

        value
    }

    /// Constructs a 27-trit word from a native integer.
    ///
    /// # Errors
    ///
    /// Returns [`WordError::OutOfRange`] if `value` is outside
    /// [`Word27::MIN_INT`] through [`Word27::MAX_INT`].
    pub fn from_int(mut value: i64) -> Result<Self, WordError> {
        if !(Self::MIN_INT..=Self::MAX_INT).contains(&value) {
            return Err(WordError::OutOfRange);
        }

        let mut trits = [Trit::Z; Self::TRIT_COUNT];

        for trit in &mut trits {
            let remainder = value.rem_euclid(3);

            *trit = match remainder {
                0 => Trit::Z,
                1 => Trit::P,
                2 => {
                    value += 1;
                    Trit::N
                }
                _ => unreachable!("remainder modulo three is always 0, 1, or 2"),
            };

            value = value.div_euclid(3);
        }

        Ok(Self(trits))
    }
}

impl From<Word27> for i64 {
    fn from(word: Word27) -> Self {
        word.to_int()
    }
}

impl TryFrom<i64> for Word27 {
    type Error = WordError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl Word54 {
    /// The minimum representable value for a 54-trit word.
    pub const MIN_INT: i128 = -(3_i128.pow(54) - 1) / 2;

    /// The maximum representable value for a 54-trit word.
    pub const MAX_INT: i128 = (3_i128.pow(54) - 1) / 2;

    /// Converts this 54-trit word to its native integer representation.
    #[must_use]
    pub fn to_int(self) -> i128 {
        let mut value = 0_i128;
        let mut place = 1_i128;

        for trit in self.0 {
            value += i128::from(trit.value()) * place;
            place *= 3;
        }

        value
    }

    /// Constructs a 54-trit word from a native integer.
    ///
    /// # Errors
    ///
    /// Returns [`WordError::OutOfRange`] if `value` is outside
    /// [`Word54::MIN_INT`] through [`Word54::MAX_INT`].
    pub fn from_int(mut value: i128) -> Result<Self, WordError> {
        if !(Self::MIN_INT..=Self::MAX_INT).contains(&value) {
            return Err(WordError::OutOfRange);
        }

        let mut trits = [Trit::Z; Self::TRIT_COUNT];

        for trit in &mut trits {
            let remainder = value.rem_euclid(3);

            *trit = match remainder {
                0 => Trit::Z,
                1 => Trit::P,
                2 => {
                    value += 1;
                    Trit::N
                }
                _ => unreachable!("remainder modulo three is always 0, 1, or 2"),
            };

            value = value.div_euclid(3);
        }

        Ok(Self(trits))
    }
}

impl From<Word54> for i128 {
    fn from(word: Word54) -> Self {
        word.to_int()
    }
}

impl TryFrom<i128> for Word54 {
    type Error = WordError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}
