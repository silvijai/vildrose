//! Fixed-width balanced ternary word types.
//!
//! Trit storage is little-endian: trit zero is the least-significant trit.
//!
//! # Common types
//!
//! - [`Tribble`] — 3 trits, range -13 through +13
//! - [`Tryte`] / [`Word9`] — 9 trits, range -9,841 through +9,841
//! - [`Word27`] — 27 trits
//! - [`Word54`] — 54 trits, used for extended-width arithmetic
//!
//! [`Word`] is generic over its trit count, allowing ISA crates to use
//! widths such as `Word<24>` and `Word<32>` without adding more core types.

mod arithmetic;
mod basic;
mod conversion;
mod cross_width;
mod logic;

use crate::trit::Trit;

/// A fixed-width balanced ternary integer containing `N` trits.
///
/// Trits are stored in little-endian order: index zero is the
/// least-significant trit.
// Add hash in the future, requires it to be added to trit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word<const N: usize>(pub(crate) [Trit; N]);

/// A 3-trit balanced ternary integer.
///
/// Range: -13 through +13.
pub type Tribble = Word<3>;

/// A 9-trit balanced ternary integer.
///
/// Range: -9,841 through +9,841.
pub type Tryte = Word<9>;

/// Alias for [`Tryte`].
pub type Word9 = Tryte;

/// A 27-trit balanced ternary integer.
pub type Word27 = Word<27>;

/// A 54-trit balanced ternary integer.
pub type Word54 = Word<54>;

/// Errors returned by fixed-width ternary-word operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordError {
    /// The supplied value cannot be represented by this word width.
    OutOfRange,
}

impl core::fmt::Display for WordError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutOfRange => {
                f.write_str("value is outside the representable balanced ternary range")
            }
        }
    }
}

impl std::error::Error for WordError {}

/// Checked division supporting potentially different operand widths.
///
/// Division returns `None` when the divisor is zero.
pub trait CheckedDiv<Rhs = Self> {
    /// The type produced by a successful division.
    type Output;

    /// Divides `self` by `rhs`, returning `None` when `rhs` is zero.
    #[must_use]
    fn checked_div(self, rhs: Rhs) -> Option<Self::Output>;
}
