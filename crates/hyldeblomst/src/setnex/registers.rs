//! Setnex scalar register-bank storage.

use crate::registers::RegisterFile;
use vildrose_core::word::Word27;
use vildrose_isa::setnex::registers::{REGISTER_COUNT, Register};

/// The architectural scalar register bank for a Setnex virtual CPU.
#[derive(Debug, Clone)]
pub struct SetnexRegisters {
    file: RegisterFile<Word27, REGISTER_COUNT>,
}

impl SetnexRegisters {
    /// Creates a scalar register bank with all registers reset to zero.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            file: RegisterFile::new([Word27::zero(); REGISTER_COUNT]),
        }
    }

    /// Returns the current value of `register`.
    ///
    /// # Panics
    ///
    /// Panics only if an internal Setnex register invariant is violated.
    #[must_use]
    pub fn read(&self, register: Register) -> Word27 {
        if register == Register::ZERO {
            Word27::zero()
        } else {
            self.file
                .read(register.number() as usize)
                .expect("Setnex Register always selects one of 27 slots")
        }
    }

    /// Writes `value` to `register`.
    ///
    /// Writes to `r0` are ignored.
    pub fn write(&mut self, register: Register, value: Word27) {
        if register != Register::ZERO {
            let written = self.file.write(register.number() as usize, value);
            debug_assert!(written);
        }
    }
}

impl Default for SetnexRegisters {
    fn default() -> Self {
        Self::new()
    }
}
