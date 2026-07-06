//! Reusable register-file storage for virtual CPU implementations.

/// Fixed-size register storage used by ISA-specific register banks.
///
/// `T` is the value stored in every register and `COUNT` is the number
/// of registers in the bank. This type deliberately contains no ISA
/// policy, such as a hardwired zero register.
#[derive(Debug, Clone)]
pub struct RegisterFile<T, const COUNT: usize> {
    values: [T; COUNT],
}

impl<T: Copy, const COUNT: usize> RegisterFile<T, COUNT> {
    /// Creates a register file from all of its initial values.
    #[must_use]
    pub const fn new(values: [T; COUNT]) -> Self {
        Self { values }
    }

    /// Returns the value stored at `index`, or `None` if it is out of range.
    #[must_use]
    pub fn read(&self, index: usize) -> Option<T> {
        self.values.get(index).copied()
    }

    /// Stores `value` at `index`.
    ///
    /// Returns `true` when `index` refers to a register, otherwise `false`.
    #[allow(clippy::option_if_let_else)]
    pub fn write(&mut self, index: usize, value: T) -> bool {
        if let Some(slot) = self.values.get_mut(index) {
            *slot = value;
            true
        } else {
            false
        }
    }

    /// Returns the number of registers in this file.
    #[must_use]
    pub const fn count(&self) -> usize {
        COUNT
    }
}
