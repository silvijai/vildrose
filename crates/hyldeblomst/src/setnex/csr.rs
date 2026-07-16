//! Setnex CSR-bank storage.

use crate::registers::RegisterFile;
use vildrose_core::word::Word27;
use vildrose_isa::setnex::csr::{CSR_COUNT, Csr};

/// The architectural CSR bank for a Setnex virtual CPU.
#[derive(Debug, Clone)]
pub struct SetnexCsrs {
    file: RegisterFile<Word27, CSR_COUNT>,
}

impl SetnexCsrs {
    /// Creates a CSR bank with all CSRs reset to zero.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            file: RegisterFile::new([Word27::zero(); CSR_COUNT]),
        }
    }

    /// Returns the current value of `csr`.
    ///
    /// # Panics
    ///
    /// Panics only if an internal CSR invariant is violated.
    #[must_use]
    pub fn read(&self, csr: Csr) -> Word27 {
        self.file
            .read(csr.index())
            .expect("Setnex CSR always selects one of 27 slots")
    }

    /// Writes `value` to `csr`.
    pub fn write(&mut self, csr: Csr, value: Word27) {
        let written = self.file.write(csr.index(), value);
        debug_assert!(written);
    }
}

impl Default for SetnexCsrs {
    fn default() -> Self {
        Self::new()
    }
}
