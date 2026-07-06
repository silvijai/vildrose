//! Fixed-width balanced ternary word types.
//!
//! # Types
//! - [`Tryte`] (alias [`Word9`]) — 9 trits, range ±9,841
//! - [`Word27`] — 27 trits, range ±3,812,798,742,493
//! - [`Word54`] — 54 trits, extended-width arithmetic value
//!
//! # Method Groups
//!
//! Each word type exposes methods for construction, sign and magnitude,
//! tritwise logic, shifts, and arithmetic through standard [`std::ops`]
//! traits and [`CheckedDiv`].
//!
//! # Cross-Width Arithmetic
//!
//! Operations between different word widths, such as `Tryte + Word27`,
//! sign-extend the narrower operand before computing. The result is always
//! the wider word type. This applies to `Add`, `Sub`, `Mul`, `Div`, and
//! [`CheckedDiv`].

use crate::ops::{ripple_add, widen};
use crate::trit::Trit;
use core::fmt;

// TODO: This file needs to be structure better, especially with the API doc in mind
// TODO: Rework errors

// <- Macros defined here
macro_rules! define_word {
    (
        $( #[$meta:meta] )*
        $name:ident($width:literal)
    ) => {
        $( #[$meta] )*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        /// A fixed-width balanced ternary word type with a width of $width trits.
        pub struct $name([Trit; $width]);

        impl_word_methods!($name, $width);
    };
}

macro_rules! impl_word_methods {
    ($name:ident, $width:literal) => {
        /// ## Construction
        impl $name {
            /// Number of trits in this word.
            pub const TRIT_COUNT: usize = $width;

            // TODO: Consider removing this or from_trits, as they are redundant with the constructor
            /// Creates a word from trits in least-significant-first order.
            pub const fn new(t: [Trit; $width]) -> Self {
                Self(t)
            }

            /// Creates a word from trits in least-significant-first order.
            pub const fn from_trits(trits: [Trit; $width]) -> Self {
                Self(trits)
            }

            /// Creates a new word filled with zeros
            pub const fn zero() -> Self {
                Self([Trit::Z; $width])
            }

            /// Returns the trit at `index`, or `None` if `index` is out of bounds.
            ///
            /// Trit index `0` is the least-significant trit.
            #[must_use]
            pub fn get_trit(&self, index: usize) -> Option<Trit> {
                self.0.get(index).copied()
            }

            /// Returns the trit at `index`.
            ///
            /// Trit index `0` is the least-significant trit.
            ///
            /// # Panics
            ///
            /// Panics if `index >= Self::TRIT_COUNT`.
            #[must_use]
            pub const fn trit(&self, index: usize) -> Trit {
                self.0[index]
            }
        }

        /// ## Sign and magnitude
        impl $name {
            /// Returns the numeric negation of this balanced ternary word.
            #[must_use]
            pub fn negate(&self) -> Self {
                Self(self.0.map(|t| t.negate()))
            }

            /// Returns the absolute (no negatives) form of the word
            #[must_use]
            pub fn abs(&self) -> Self {
                if self.sign() == Trit::N {
                    self.negate()
                } else {
                    *self
                }
            }

            /// Returns the sign (whether it's negative, positive or zero) of the word
            #[must_use]
            pub fn sign(&self) -> Trit {
                for t in self.0.iter().rev() {
                    if *t != Trit::Z {
                        return *t;
                    }
                }
                Trit::Z
            }
        }

        /// ## Tritwise logic
        impl $name {
            /// Returns the component-wise minimum of two words.
            #[must_use]
            pub fn tmin(&self, rhs: Self) -> Self {
                Self(std::array::from_fn(|i| self.0[i].tmin(rhs.0[i])))
            }

            /// Returns the component-wise maximum of two words.
            #[must_use]
            pub fn tmax(&self, rhs: Self) -> Self {
                Self(std::array::from_fn(|i| self.0[i].tmax(rhs.0[i])))
            }

            /// Returns the tritwise ternary negation of this word.
            ///
            /// Each `N` becomes `P`, each `P` becomes `N`, and `Z` remains `Z`.
            #[must_use]
            pub fn tnot(&self) -> Self {
                Self(self.0.map(|t| t.negate()))
            }

            /// Returns the tritwise clipping of the word
            #[must_use]
            pub fn tclip(&self) -> Self {
                Self(self.0.map(|t| t.clip()))
            }

            /// Returns the word-valued numeric sign: `-1`, `0`, or `+1`.
            #[must_use]
            pub fn signum(&self) -> Self {
                Self(std::array::from_fn(|index| {
                    if index == 0 { self.sign() } else { Trit::Z }
                }))
            }

            /// Returns the tritwise consensus of two words.
            #[must_use]
            pub fn tconsensus(&self, rhs: Self) -> Self {
                Self(std::array::from_fn(|i| self.0[i].consensus(rhs.0[i])))
            }
        }

        /// ## Shifts
        impl $name {
            /// Trit shift left by n positions, filling with zeros
            #[must_use]
            pub fn tshl(&self, count: usize) -> Self {
                Self(std::array::from_fn(|index| {
                    if index < count {
                        Trit::Z
                    } else {
                        self.0[index - count]
                    }
                }))
            }

            /// Returns the arithmetic right shift by `count` trits.
            ///
            /// Vacated most-significant trits are filled with the word's sign.
            /// If `count >= Self::TRIT_COUNT`, every trit is filled with the sign.
            #[must_use]
            pub fn tshr(&self, count: usize) -> Self {
                let sign = self.sign();

                if count >= Self::TRIT_COUNT {
                    return Self([sign; Self::TRIT_COUNT]);
                }

                Self(std::array::from_fn(|index| {
                    let source = index + count;

                    if source < Self::TRIT_COUNT {
                        self.0[source]
                    } else {
                        sign
                    }
                }))
            }

            /// Returns the logical right shift by `count` trits.
            ///
            /// Vacated most-significant trits are filled with zero.
            /// If `count >= Self::TRIT_COUNT`, returns zero.
            #[must_use]
            pub fn tlshr(&self, count: usize) -> Self {
                if count >= Self::TRIT_COUNT {
                    return Self::zero();
                }

                Self(std::array::from_fn(|index| {
                    let source = index + count;

                    if source < Self::TRIT_COUNT {
                        self.0[source]
                    } else {
                        Trit::Z
                    }
                }))
            }
        }

        impl std::ops::Neg for $name {
            type Output = $name;
            fn neg(self) -> $name {
                self.negate()
            }
        }

        // Manually implemented, since the ordering is opposite to what Ord expects. As least significant trit is at index 0 (A.K.A. little-endian)
        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                for index in (0..Self::TRIT_COUNT).rev() {
                    match self.0[index].cmp(&other.0[index]) {
                        std::cmp::Ordering::Equal => {}
                        ordering => return ordering,
                    }
                }

                std::cmp::Ordering::Equal
            }
        }

        /// Following most to least significant ordering
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // Reverse here to follow most to least
                for t in self.0.iter().rev() {
                    write!(f, "{}", t)?;
                }
                Ok(())
            }
        }
    };
}

// <- Errors
/// Errors returned by fixed-width ternary word operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordError {
    /// A host integer cannot be represented by the requested word width.
    OutOfRange,
}

impl fmt::Display for WordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange => f.write_str("value is outside the representable range"),
        }
    }
}

impl std::error::Error for WordError {}

// <- Definitions
define_word! { Tryte(9) }

define_word! { Word27(27) }

define_word! { Word54(54) }

// <- Int integrations
/// Checked division that can fail on divide-by-zero or produce a wider `Output` when dividing across word widths.
pub trait CheckedDiv<Rhs = Self> {
    /// The result type of the division — usually `Self`, but the wider type when dividing across widths.
    type Output;
    /// Divides `self` by `rhs`, returning `None` on division by zero.
    fn checked_div(self, rhs: Rhs) -> Option<Self::Output>;
}

/// Logic used for int related conversions and logic. Used to define which int each word size needs.
impl Tryte {
    /// The minimum representable value for this word width, as a native integer.
    pub const MIN_INT: i16 = -(3_i16.pow(9) - 1) / 2;
    /// The maximum representable value for this word width, as a native integer.
    pub const MAX_INT: i16 = (3_i16.pow(9) - 1) / 2;

    /// Converts this word to its native integer representation.
    pub fn to_int(self) -> i16 {
        let mut val: i16 = 0;
        let mut place: i16 = 1;

        for t in &self.0 {
            val += i16::from(t.value()) * place;
            place *= 3;
        }

        val
    }

    /// Constructs a word from a native integer.
    ///
    /// # Errors
    ///
    /// Returns `Err` if `val` is outside the representable range for this word width,
    /// i.e. outside `[Self::MIN_INT, Self::MAX_INT]`.
    pub fn from_int(mut val: i16) -> Result<Self, WordError> {
        if !(Self::MIN_INT..=Self::MAX_INT).contains(&val) {
            return Err(WordError::OutOfRange);
        }

        let mut trits = [Trit::Z; Self::TRIT_COUNT];

        for trit in &mut trits {
            let rem = val.rem_euclid(3);

            *trit = if rem == 2 {
                val += 1;
                Trit::N
            } else if rem == 1 {
                Trit::P
            } else {
                Trit::Z
            };

            val = val.div_euclid(3);
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
    /// The minimum representable value for this word width, as a native integer.
    pub const MIN_INT: i64 = -(3_i64.pow(27) - 1) / 2;
    /// The maximum representable value for this word width, as a native integer.
    pub const MAX_INT: i64 = (3_i64.pow(27) - 1) / 2;

    /// Converts this word to its native integer representation.
    pub fn to_int(self) -> i64 {
        let mut val: i64 = 0;
        let mut place: i64 = 1;

        for t in &self.0 {
            val += i64::from(t.value()) * place;
            place *= 3;
        }

        val
    }

    /// Constructs a word from a native integer.
    ///
    /// # Errors
    ///
    /// Returns `Err` if `val` is outside the representable range for this word width,
    /// i.e. outside `[Self::MIN_INT, Self::MAX_INT]`.
    pub fn from_int(mut val: i64) -> Result<Self, WordError> {
        if !(Self::MIN_INT..=Self::MAX_INT).contains(&val) {
            return Err(WordError::OutOfRange);
        }

        let mut trits = [Trit::Z; Self::TRIT_COUNT];

        for trit in &mut trits {
            let rem = val.rem_euclid(3);

            *trit = if rem == 2 {
                val += 1;
                Trit::N
            } else if rem == 1 {
                Trit::P
            } else {
                Trit::Z
            };

            val = val.div_euclid(3);
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
    /// The minimum representable value for this word width, as a native integer.
    pub const MIN_INT: i128 = -(3_i128.pow(54) - 1) / 2;
    /// The maximum representable value for this word width, as a native integer.
    pub const MAX_INT: i128 = (3_i128.pow(54) - 1) / 2;

    /// Converts this word to its native integer representation.
    pub fn to_int(self) -> i128 {
        let mut val: i128 = 0;
        let mut place: i128 = 1;

        for t in &self.0 {
            val += i128::from(t.value()) * place;
            place *= 3;
        }

        val
    }

    /// Constructs a word from a native integer.
    ///
    /// # Errors
    ///
    /// Returns `Err` if `val` is outside the representable range for this word width,
    /// i.e. outside `[Self::MIN_INT, Self::MAX_INT]`.
    pub fn from_int(mut val: i128) -> Result<Self, WordError> {
        if !(Self::MIN_INT..=Self::MAX_INT).contains(&val) {
            return Err(WordError::OutOfRange);
        }

        let mut trits = [Trit::Z; Self::TRIT_COUNT];

        for trit in &mut trits {
            let rem = val.rem_euclid(3);

            *trit = if rem == 2 {
                val += 1;
                Trit::N
            } else if rem == 1 {
                Trit::P
            } else {
                Trit::Z
            };

            val = val.div_euclid(3);
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

// <- Arithmetic implementations same width
macro_rules! impl_arithmetic_same_width {
    ($T:ident) => {
        impl std::ops::Add for $T {
            type Output = $T;
            fn add(self, rhs: $T) -> $T {
                $T(ripple_add(self.0, rhs.0))
            }
        }

        impl std::ops::Sub for $T {
            type Output = $T;
            fn sub(self, rhs: $T) -> $T {
                self + (-rhs)
            }
        }

        impl std::ops::Mul for $T {
            type Output = $T;
            fn mul(self, rhs: $T) -> $T {
                let mut product = $T::zero();
                for (i, t) in rhs.0.iter().enumerate() {
                    let shifted = self.tshl(i);
                    product = match t {
                        Trit::Z => product,
                        Trit::P => product + shifted,
                        Trit::N => product + shifted.negate(),
                    };
                }
                product
            }
        }

        impl CheckedDiv for $T {
            type Output = $T;
            fn checked_div(self, rhs: Self) -> Option<$T> {
                if rhs == Self::zero() {
                    return None;
                }
                Self::from_int(self.to_int() / rhs.to_int()).ok()
            }
        }

        impl std::ops::Div for $T {
            type Output = $T;
            fn div(self, rhs: $T) -> $T {
                self.checked_div(rhs)
                    .unwrap_or_else(|| panic!("{}: division by zero", stringify!($T)))
            }
        }
    };
}

impl_arithmetic_same_width!(Tryte);
impl_arithmetic_same_width!(Word27);
impl_arithmetic_same_width!(Word54);

macro_rules! impl_arithmetic_cross_width {
    ($Narrow:ident, $NarrowW:literal, $Wide:ident, $WideW:literal) => {
        // Addition
        impl std::ops::Add<$Wide> for $Narrow {
            type Output = $Wide;
            fn add(self, rhs: $Wide) -> $Wide {
                $Wide(widen::<$NarrowW, $WideW>(self.0)) + rhs
            }
        }

        impl std::ops::Add<$Narrow> for $Wide {
            type Output = $Wide;
            fn add(self, rhs: $Narrow) -> $Wide {
                self + $Wide(widen::<$NarrowW, $WideW>(rhs.0))
            }
        }

        // Subtraction
        impl std::ops::Sub<$Wide> for $Narrow {
            type Output = $Wide;
            fn sub(self, rhs: $Wide) -> $Wide {
                $Wide(widen::<$NarrowW, $WideW>(self.0)) - rhs
            }
        }

        impl std::ops::Sub<$Narrow> for $Wide {
            type Output = $Wide;
            fn sub(self, rhs: $Narrow) -> $Wide {
                self - $Wide(widen::<$NarrowW, $WideW>(rhs.0))
            }
        }

        // Multiplication
        impl std::ops::Mul<$Wide> for $Narrow {
            type Output = $Wide;
            fn mul(self, rhs: $Wide) -> $Wide {
                $Wide(widen::<$NarrowW, $WideW>(self.0)) * rhs
            }
        }

        impl std::ops::Mul<$Narrow> for $Wide {
            type Output = $Wide;
            fn mul(self, rhs: $Narrow) -> $Wide {
                self * $Wide(widen::<$NarrowW, $WideW>(rhs.0))
            }
        }

        // Checked division implemented separately, most applicable for vildrose-VM
        impl CheckedDiv<$Wide> for $Narrow {
            type Output = $Wide;
            fn checked_div(self, rhs: $Wide) -> Option<$Wide> {
                $Wide(widen::<$NarrowW, $WideW>(self.0)).checked_div(rhs)
            }
        }

        impl CheckedDiv<$Narrow> for $Wide {
            type Output = $Wide;
            fn checked_div(self, rhs: $Narrow) -> Option<$Wide> {
                self.checked_div($Wide(widen::<$NarrowW, $WideW>(rhs.0)))
            }
        }

        // Division
        impl std::ops::Div<$Wide> for $Narrow {
            type Output = $Wide;
            fn div(self, rhs: $Wide) -> $Wide {
                $Wide(widen::<$NarrowW, $WideW>(self.0)) / rhs
            }
        }

        impl std::ops::Div<$Narrow> for $Wide {
            type Output = $Wide;
            fn div(self, rhs: $Narrow) -> $Wide {
                self / $Wide(widen::<$NarrowW, $WideW>(rhs.0))
            }
        }
    };
}

impl_arithmetic_cross_width!(Tryte, 9, Word27, 27);
impl_arithmetic_cross_width!(Tryte, 9, Word54, 54);
impl_arithmetic_cross_width!(Word27, 27, Word54, 54);

/// Word9 definition, it's just an alias for Tryte
pub type Word9 = Tryte;
