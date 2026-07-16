//! Setnex control/status register identifiers.

use vildrose_core::word::Tribble;

/// The number of architectural CSRs addressable by one tribble.
pub const CSR_COUNT: usize = 27;

/// A Setnex CSR address.
///
/// CSR addresses are balanced-ternary tribbles in the range -13 through +13.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Csr(Tribble);

impl Csr {
    /// Creates a CSR from a tribble address.
    #[must_use]
    pub const fn new(address: Tribble) -> Self {
        Self(address)
    }

    /// Returns the ternary CSR address.
    #[must_use]
    pub const fn address(self) -> Tribble {
        self.0
    }

    /// Returns the dense storage index for this CSR, in `0..CSR_COUNT`.
    ///
    /// # Panics
    ///
    /// Panics only if the internal tribble-to-index invariant is violated.
    #[must_use]
    pub fn index(self) -> usize {
        let offset = self.0.to_int() + 13;
        usize::try_from(offset)
            .ok()
            .filter(|&index| index < CSR_COUNT)
            .expect("Setnex CSR address always maps to one of 27 slots")
    }

    /// Creates a CSR from an integer address in `-13..=13`.
    #[must_use]
    pub fn from_int(value: i8) -> Option<Self> {
        i32::from(value)
            .try_into()
            .ok()
            .and_then(|value| Tribble::from_int(value).ok())
            .map(Self)
    }
}
