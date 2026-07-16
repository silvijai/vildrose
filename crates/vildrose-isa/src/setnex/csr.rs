//! Setnex control/status register identifiers.
//!
//! CSR addresses are T3 (3 trits), providing 27 addressable slots
//! with integer values in -13..=13. Each CSR is a full T27 word.
//! Unused slots are reserved and read as zero.
//
//! See setnex-spec §2.2 for the architectural CSR map.

use vildrose_core::trit::Trit;
use vildrose_core::word::Tribble;

/// The number of architectural CSRs addressable by one tribble.
pub const CSR_COUNT: usize = 27;

/// A Setnex CSR address.
///
/// CSR addresses are balanced-ternary tribbles in the range -13 through +13.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Csr(Tribble);

impl Csr {
    // Positive addresses refer to user code and data.
    // Negative addresses refer to stack and kernel space
    // Using three trits for definition, rather than from_int(), since it's const methods
    /// Address 0 0 + (decimal 1): Program counter (T27, word address).
    pub const PC: Self = Self(Tribble::new([Trit::P, Trit::Z, Trit::Z]));

    /// Address 0 + – (decimal 2): Logic mode (see §6).
    pub const LMODE: Self = Self(Tribble::new([Trit::N, Trit::P, Trit::Z]));

    /// Address 0 + 0 (decimal 3): Arithmetic flags (see §5.5).
    pub const FLAGS: Self = Self(Tribble::new([Trit::Z, Trit::P, Trit::Z]));

    /// Address 0 + + (decimal 4): Exception program counter.
    pub const EPC: Self = Self(Tribble::new([Trit::P, Trit::P, Trit::Z]));

    /// Address + – – (decimal 5): Exception cause (T27).
    pub const ECAUSE: Self = Self(Tribble::new([Trit::N, Trit::N, Trit::P]));

    /// Address + – 0 (decimal 6): Exception vector (handler address).
    pub const EVEC: Self = Self(Tribble::new([Trit::Z, Trit::N, Trit::P]));

    /// Address + – + (decimal 7): Processor status (see §2.4).
    pub const STATUS: Self = Self(Tribble::new([Trit::P, Trit::N, Trit::P]));

    /// Address + 0 – (decimal 8): Saved STATUS on exception entry.
    pub const ESAVE: Self = Self(Tribble::new([Trit::N, Trit::Z, Trit::P]));

    /// Address + 0 0 (decimal 9): Exception trap value (see §8.5).
    pub const ETVAL: Self = Self(Tribble::new([Trit::Z, Trit::Z, Trit::P]));

    /// Address + 0 + (decimal 10): Frame-2 exception PC (nested).
    pub const EPC2: Self = Self(Tribble::new([Trit::P, Trit::Z, Trit::P]));

    /// Address + + – (decimal 11): Frame-2 exception cause.
    pub const ECAUSE2: Self = Self(Tribble::new([Trit::N, Trit::P, Trit::P]));

    /// Address + + 0 (decimal 12): Frame-2 saved STATUS.
    pub const ESAVE2: Self = Self(Tribble::new([Trit::Z, Trit::P, Trit::P]));

    /// Address + + + (decimal 13): Frame-2 exception trap value.
    pub const ETVAL2: Self = Self(Tribble::new([Trit::P, Trit::P, Trit::P]));

    /// Address 0 0 – (decimal -1): MPU region index selector.
    pub const MPU_SELECT: Self = Self(Tribble::new([Trit::N, Trit::Z, Trit::Z]));

    /// Address 0 – + (decimal -2): Base address of the selected MPU region.
    pub const MPU_BASE: Self = Self(Tribble::new([Trit::P, Trit::N, Trit::Z]));

    /// Address 0 – 0 (decimal -3): Config (size + permissions + valid) of selected region.
    pub const MPU_CFG: Self = Self(Tribble::new([Trit::Z, Trit::N, Trit::Z]));

    /// Address 0 – – (decimal -4): Pending-IRQ bitvector.
    pub const IPENDING: Self = Self(Tribble::new([Trit::N, Trit::N, Trit::Z]));

    /// Address – + + (decimal -5): Per-line IRQ enable mask.
    pub const IENABLE: Self = Self(Tribble::new([Trit::P, Trit::P, Trit::N]));

    /// Address – + 0 (decimal -6): Per-line IRQ priority.
    pub const IPRIORITY: Self = Self(Tribble::new([Trit::Z, Trit::P, Trit::N]));

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
        // map integer address -13..=13 to 0..=26
        let offset = self.0.to_int() + 13;
        usize::try_from(offset)
            .ok()
            .filter(|&index| index < CSR_COUNT)
            .expect("Setnex CSR address always maps to one of 27 slots")
    }

    /// Creates a CSR from an integer address in `-13..=13`.
    #[must_use]
    pub fn from_int(value: i8) -> Option<Self> {
        Tribble::from_int(value).ok().map(Self)
    }
}
