//! Fixed-width balanced ternary word types.
//!
//! # Types
//! - [`Tryte`] (alias [`Word9`]) — 9 trits, range ±9,841
//! - [`Word27`] — 27 trits, range ±193,710,244
//! - [`Word54`] — 54 trits, primary register width
//!
//! # Method Groups
//! Each word type exposes methods grouped as: construction, sign/magnitude,
//! tritwise logic, shifts, and arithmetic (via [`std::ops`] traits
//! and [`WordType`]/[`CheckedDiv`]).
//!
//! # Cross-Width Arithmetic
//!
//! Operations between different word widths (e.g. `Tryte + Word27`) widen the
//! narrower operand before performing the operation; the result is always the
//! wider type. This applies uniformly to `Add`, `Sub`, `Mul`, `Div`, and `CheckedDiv`.

use crate::ops::{ripple_add, widen};
use crate::trit::Trit;

// TODO: This file needs to be structure better, especially with the API doc in mind

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
            /// Creates a new word filled with zeros
            pub const fn new(t: [Trit; $width]) -> Self {
                Self(t)
            }

            /// Creates a new word from an array of trits
            pub const fn from_trits(trits: [Trit; $width]) -> Self {
                Self(trits)
            }

            /// Creates a new word filled with zeros
            pub const fn zero() -> Self {
                Self([Trit::Z; $width])
            }

            /// Returns the trit at the given index, panicking if the index is out of bounds.
            pub fn trit(&self, i: usize) -> Trit {
                if i < $width {
                    self.0[i]
                } else {
                    panic!("Index out of bounds for {}: {}", stringify!($name), i);
                }
            }
        }

        /// ## Sign and magnitude
        impl $name {
            /// Returns the inverted (negated) form of the word
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
            /// Returns the minimum of two words, element-wise
            #[must_use]
            pub fn tmin(&self, rhs: Self) -> Self {
                Self(std::array::from_fn(|i| self.0[i].tmin(rhs.0[i])))
            }

            /// Returns the maximum of two words, element-wise
            #[must_use]
            pub fn tmax(&self, rhs: Self) -> Self {
                Self(std::array::from_fn(|i| self.0[i].tmax(rhs.0[i])))
            }

            /// Returns the tritwise negation of the word
            #[must_use]
            pub fn tnot(&self) -> Self {
                Self(self.0.map(|t| t.negate()))
            }

            /// Returns the tritwise clipping of the word
            #[must_use]
            pub fn tclip(&self) -> Self {
                Self(self.0.map(|t| t.clip()))
            }

            /// Returns the tritwise sign of the word
            #[must_use]
            pub fn tsign(&self) -> Self {
                let s = self.sign();
                Self(std::array::from_fn(|i| if i == 0 { s } else { Trit::Z }))
            }

            /// Returns the tritwise consensus of two words
            #[must_use]
            pub fn tconsensus(&self, rhs: Self) -> Self {
                Self(std::array::from_fn(|i| self.0[i].consensus(rhs.0[i])))
            }
        }

        /// ## Shifts
        impl $name {
            /// Trit shift left by n positions, filling with zeros
            #[must_use]
            pub fn tshl(&self, n: usize) -> Self {
                Self(std::array::from_fn(|i| {
                    if i < n { Trit::Z } else { self.0[i - n] }
                }))
            }

            /// Trit shift right by n positions, filling with the sign of the word
            #[must_use]
            pub fn tshr(&self, n: usize) -> Self {
                let sign = self.sign();
                Self(std::array::from_fn(|i| {
                    if i + n < $width { self.0[i + n] } else { sign }
                }))
            }

            /// Trit logical shift right by n positions, filling with zeros
            #[must_use]
            pub fn tlshr(&self, n: usize) -> Self {
                Self(std::array::from_fn(|i| {
                    if i + n < $width {
                        self.0[i + n]
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
                self.to_int().cmp(&other.to_int())
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

// <- Int integrations
/// Trait used for int related conversions and logic. Used to define which int each word size needs.
pub trait WordType: Sized {
    /// The native integer type used to represent te specific word's value (e.g. i16 for Tryte)
    type Int;
    /// The minimum representable value for this word width, as a native integer.
    const MIN_INT: Self::Int;
    /// The maximum representable value for this word width, as a native integer.
    const MAX_INT: Self::Int;
    /// Converts this word to its native integer representation.
    fn to_int(self) -> Self::Int;
    /// Constructs a word from a native integer.
    ///
    /// # Errors
    ///
    /// Returns `Err` if `val` is outside the representable range for this word width,
    /// i.e. outside `[Self::MIN_INT, Self::MAX_INT]`.
    fn from_int(val: Self::Int) -> Result<Self, &'static str>;
}

/// Checked division that can fail on divide-by-zero or produce a wider `Output` when dividing across word widths.
pub trait CheckedDiv<Rhs = Self> {
    /// The result type of the division — usually `Self`, but the wider type when dividing across widths.
    type Output;
    /// Divides `self` by `rhs`, returning `None` on division by zero.
    fn checked_div(self, rhs: Rhs) -> Option<Self::Output>;
}

macro_rules! impl_word_conversions {
    ($name:ident, $width:literal, $int:ty) => {
        impl WordType for $name {
            type Int = $int;

            const MAX_INT: $int = ((3 as $int).pow($width) - 1) / 2;
            const MIN_INT: $int = -Self::MAX_INT;

            fn to_int(self) -> $int {
                let mut val: $int = 0;
                let mut place: $int = 1;
                for t in self.0.iter() {
                    val += t.value() as $int * place;
                    place *= 3;
                }
                val
            }

            fn from_int(mut val: $int) -> Result<Self, &'static str> {
                if !(Self::MIN_INT..=Self::MAX_INT).contains(&val) {
                    return Err(concat!(stringify!($name), ": value out of range"));
                }
                let mut trits = [Trit::Z; $width];
                for i in 0..$width {
                    let rem = val.rem_euclid(3);
                    trits[i] = if rem == 2 {
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

        impl From<$name> for $int {
            fn from(word: $name) -> $int {
                word.to_int()
            }
        }

        impl TryFrom<$int> for $name {
            type Error = &'static str;
            fn try_from(val: $int) -> Result<Self, Self::Error> {
                Self::from_int(val)
            }
        }
    };
}

// <- Definitions
define_word! { Tryte(9) }
impl_word_conversions!(Tryte, 9, i16);

define_word! { Word27(27) }
impl_word_conversions!(Word27, 27, i64);

define_word! { Word54(54) }
impl_word_conversions!(Word54, 54, i128);

// <- Arithmetic implementations
macro_rules! impl_add_same_width {
    ($T:ident) => {
        impl std::ops::Add for $T {
            type Output = $T;
            fn add(self, rhs: $T) -> $T {
                $T(ripple_add(self.0, rhs.0))
            }
        }
    };
}

impl_add_same_width!(Tryte);
impl_add_same_width!(Word27);
impl_add_same_width!(Word54);

macro_rules! impl_sub_same_width {
    ($T:ident) => {
        impl std::ops::Sub for $T {
            type Output = $T;
            fn sub(self, rhs: $T) -> $T {
                self + (-rhs)
            }
        }
    };
}

impl_sub_same_width!(Tryte);
impl_sub_same_width!(Word27);
impl_sub_same_width!(Word54);

macro_rules! impl_mul_same_width {
    ($T:ident) => {
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
    };
}

impl_mul_same_width!(Tryte);
impl_mul_same_width!(Word27);
impl_mul_same_width!(Word54);

/// Does division with safety around zero
macro_rules! impl_checked_div_same_width {
    ($T:ident) => {
        impl CheckedDiv for $T {
            type Output = $T;
            fn checked_div(self, rhs: Self) -> Option<$T> {
                if rhs == Self::zero() {
                    return None;
                }
                Self::from_int(self.to_int() / rhs.to_int()).ok()
            }
        }
    };
}

impl_checked_div_same_width!(Tryte);
impl_checked_div_same_width!(Word27);
impl_checked_div_same_width!(Word54);

/// Division with panic on zero, same as rust's standard integers
macro_rules! impl_div_same_width {
    ($T:ident) => {
        impl std::ops::Div for $T {
            type Output = $T;
            fn div(self, rhs: $T) -> $T {
                self.checked_div(rhs)
                    .unwrap_or_else(|| panic!("{}: division by zero or overflow", stringify!($T)))
            }
        }
    };
}

impl_div_same_width!(Tryte);
impl_div_same_width!(Word27);
impl_div_same_width!(Word54);

macro_rules! impl_add_cross_width {
    ($Narrow:ident, $NarrowW:literal, $Wide:ident, $WideW:literal) => {
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
    };
}

impl_add_cross_width!(Tryte, 9, Word27, 27);
impl_add_cross_width!(Tryte, 9, Word54, 54);
impl_add_cross_width!(Word27, 27, Word54, 54);

macro_rules! impl_sub_cross_width {
    ($Narrow:ident, $NarrowW:literal, $Wide:ident, $WideW:literal) => {
        /// Widens the narrower operand before subtracting; result is always the wider type.
        impl std::ops::Sub<$Wide> for $Narrow {
            type Output = $Wide;
            fn sub(self, rhs: $Wide) -> $Wide {
                $Wide(widen::<$NarrowW, $WideW>(self.0)) - rhs
            }
        }

        /// Widens the narrower operand before subtracting; result is always the wider type.
        impl std::ops::Sub<$Narrow> for $Wide {
            type Output = $Wide;
            fn sub(self, rhs: $Narrow) -> $Wide {
                self - $Wide(widen::<$NarrowW, $WideW>(rhs.0))
            }
        }
    };
}

impl_sub_cross_width!(Tryte, 9, Word27, 27);
impl_sub_cross_width!(Tryte, 9, Word54, 54);
impl_sub_cross_width!(Word27, 27, Word54, 54);

macro_rules! impl_mul_cross_width {
    ($Narrow:ident, $NarrowW:literal, $Wide:ident, $WideW:literal) => {
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
    };
}

impl_mul_cross_width!(Tryte, 9, Word27, 27);
impl_mul_cross_width!(Tryte, 9, Word54, 54);
impl_mul_cross_width!(Word27, 27, Word54, 54);

// Checked division implemented separately, most applicable for vildrose-VM
macro_rules! impl_checked_div_cross_width {
    ($Narrow:ident, $NarrowW:literal, $Wide:ident, $WideW:literal) => {
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
    };
}

impl_checked_div_cross_width!(Tryte, 9, Word27, 27);
impl_checked_div_cross_width!(Tryte, 9, Word54, 54);
impl_checked_div_cross_width!(Word27, 27, Word54, 54);

macro_rules! impl_div_cross_width {
    ($Narrow:ident, $NarrowW:literal, $Wide:ident, $WideW:literal) => {
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

impl_div_cross_width!(Tryte, 9, Word27, 27);
impl_div_cross_width!(Tryte, 9, Word54, 54);
impl_div_cross_width!(Word27, 27, Word54, 54);

/// Word9 definition, it's just an alias for Tryte
pub type Word9 = Tryte;
