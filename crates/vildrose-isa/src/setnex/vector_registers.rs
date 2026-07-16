//! Setnex vector-register definitions and aliases.

/// The number of vector registers in Setnex.
pub const VECTOR_REGISTER_COUNT: usize = 27;

/// A physical register in the Setnex vector register bank.
///
/// Setnex has 27 vector registers, numbered `v0` through `v26`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VectorRegister(u8);

impl VectorRegister {
    /// All physical vector registers in architectural order, `v0` through `v26`.
    pub const ALL: [Self; VECTOR_REGISTER_COUNT] = [
        Self(0),
        Self(1),
        Self(2),
        Self(3),
        Self(4),
        Self(5),
        Self(6),
        Self(7),
        Self(8),
        Self(9),
        Self(10),
        Self(11),
        Self(12),
        Self(13),
        Self(14),
        Self(15),
        Self(16),
        Self(17),
        Self(18),
        Self(19),
        Self(20),
        Self(21),
        Self(22),
        Self(23),
        Self(24),
        Self(25),
        Self(26),
    ];

    /// Returns the register with architectural number `number`.
    ///
    /// Returns `None` unless `number` is in `0..VECTOR_REGISTER_COUNT`.
    #[must_use]
    pub const fn new(number: u8) -> Option<Self> {
        if (number as usize) < VECTOR_REGISTER_COUNT {
            Some(Self(number))
        } else {
            None
        }
    }

    /// Returns this register's architectural number in `0..VECTOR_REGISTER_COUNT`.
    #[must_use]
    pub const fn number(self) -> u8 {
        self.0
    }
}

#[allow(missing_docs)]
impl VectorRegister {
    pub const V0: Self = Self(0);
    pub const V1: Self = Self(1);
    pub const V2: Self = Self(2);
    pub const V3: Self = Self(3);
    pub const V4: Self = Self(4);
    pub const V5: Self = Self(5);
    pub const V6: Self = Self(6);
    pub const V7: Self = Self(7);
    pub const V8: Self = Self(8);
    pub const V9: Self = Self(9);
    pub const V10: Self = Self(10);
    pub const V11: Self = Self(11);
    pub const V12: Self = Self(12);
    pub const V13: Self = Self(13);
    pub const V14: Self = Self(14);
    pub const V15: Self = Self(15);
    pub const V16: Self = Self(16);
    pub const V17: Self = Self(17);
    pub const V18: Self = Self(18);
    pub const V19: Self = Self(19);
    pub const V20: Self = Self(20);
    pub const V21: Self = Self(21);
    pub const V22: Self = Self(22);
    pub const V23: Self = Self(23);
    pub const V24: Self = Self(24);
    pub const V25: Self = Self(25);
    pub const V26: Self = Self(26);
}
