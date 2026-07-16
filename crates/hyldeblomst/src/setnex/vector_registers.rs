//! Setnex vector register-bank storage.

use crate::registers::RegisterFile;
use vildrose_core::trit::Trit;
use vildrose_isa::setnex::vector_registers::{VECTOR_REGISTER_COUNT, VectorRegister};

/// The number of trit lanes in one Setnex vector register.
pub const VECTOR_LANE_COUNT: usize = 27;

/// The value stored in one Setnex vector register.
pub type VectorWord = [Trit; VECTOR_LANE_COUNT];

/// The architectural vector register bank for a Setnex virtual CPU.
#[derive(Debug, Clone)]
pub struct SetnexVectorRegisters {
    file: RegisterFile<VectorWord, VECTOR_REGISTER_COUNT>,
}

impl SetnexVectorRegisters {
    /// Creates a vector register bank with all registers reset to zero.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            file: RegisterFile::new([[Trit::Z; VECTOR_LANE_COUNT]; VECTOR_REGISTER_COUNT]),
        }
    }

    /// Returns the current value of `register`.
    ///
    /// # Panics
    ///
    /// Panics only if an internal Setnex register invariant is violated.
    #[must_use]
    pub fn read(&self, register: VectorRegister) -> VectorWord {
        self.file
            .read(register.number() as usize)
            .expect("Setnex vector register always selects one of 27 slots")
    }

    /// Writes `value` to `register`.
    pub fn write(&mut self, register: VectorRegister, value: VectorWord) {
        let written = self.file.write(register.number() as usize, value);
        debug_assert!(written);
    }
}

impl Default for SetnexVectorRegisters {
    fn default() -> Self {
        Self::new()
    }
}
