//! Setnex scalar-register definitions and ABI aliases.

/// The number of scalar general-purpose registers in Setnex.
pub const REGISTER_COUNT: usize = 27;

/// A physical register in the Setnex scalar register bank.
///
/// Setnex has 27 scalar registers, numbered `r0` through `r26`. ABI aliases
/// such as [`Self::SP`] and [`Self::A0`] refer to the same physical register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Register(u8);

impl Register {
    /// All physical scalar registers in architectural order, `r0` through `r26`.
    pub const ALL: [Self; REGISTER_COUNT] = [
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

    /// Zero register `r0`.
    pub const ZERO: Self = Self(0);

    /// Return-address register `ra`, architectural `r1`.
    pub const RA: Self = Self(1);

    /// Stack-pointer register `sp`, architectural `r2`.
    pub const SP: Self = Self(2);

    /// Global-pointer register `gp`, architectural `r3`.
    pub const GP: Self = Self(3);

    /// Thread-pointer register `tp`, architectural `r4`.
    pub const TP: Self = Self(4);

    /// Temporary register `t0`, architectural `r5`.
    pub const T0: Self = Self(5);

    /// Temporary register `t1`, architectural `r6`.
    pub const T1: Self = Self(6);

    /// Temporary register `t2`, architectural `r7`.
    pub const T2: Self = Self(7);

    /// Frame-pointer and saved register `s0`, architectural `r8`.
    pub const S0: Self = Self(8);

    /// Saved register `s1`, architectural `r9`.
    pub const S1: Self = Self(9);

    /// Argument and primary return-value register `a0`, architectural `r10`.
    pub const A0: Self = Self(10);

    /// Argument register `a1`, architectural `r11`.
    pub const A1: Self = Self(11);

    /// Argument register `a2`, architectural `r12`.
    pub const A2: Self = Self(12);

    /// Argument register `a3`, architectural `r13`.
    pub const A3: Self = Self(13);

    /// Argument register `a4`, architectural `r14`.
    pub const A4: Self = Self(14);

    /// Argument register `a5`, architectural `r15`.
    pub const A5: Self = Self(15);

    /// Argument register `a6`, architectural `r16`.
    pub const A6: Self = Self(16);

    /// Syscall-number and argument register `a7`, architectural `r17`.
    pub const A7: Self = Self(17);

    /// Saved register `s2`, architectural `r18`.
    pub const S2: Self = Self(18);

    /// Saved register `s3`, architectural `r19`.
    pub const S3: Self = Self(19);

    /// Saved register `s4`, architectural `r20`.
    pub const S4: Self = Self(20);

    /// Saved register `s5`, architectural `r21`.
    pub const S5: Self = Self(21);

    /// Saved register `s6`, architectural `r22`.
    pub const S6: Self = Self(22);

    /// Saved register `s7`, architectural `r23`.
    pub const S7: Self = Self(23);

    /// Saved register `s8`, architectural `r24`.
    pub const S8: Self = Self(24);

    /// Saved register `s9`, architectural `r25`.
    pub const S9: Self = Self(25);

    /// Temporary register `t3`, architectural `r26`.
    pub const T3: Self = Self(26);

    /// Returns the register with architectural number `number`.
    ///
    /// Returns `None` unless `number` is in `0..REGISTER_COUNT`.
    #[must_use]
    pub const fn new(number: u8) -> Option<Self> {
        if (number as usize) < REGISTER_COUNT {
            Some(Self(number))
        } else {
            None
        }
    }

    /// Returns this register's architectural number in `0..REGISTER_COUNT`.
    #[must_use]
    pub const fn number(self) -> u8 {
        self.0
    }
}

#[allow(missing_docs)]
impl Register {
    pub const R0: Self = Self(0);
    pub const R1: Self = Self(1);
    pub const R2: Self = Self(2);
    pub const R3: Self = Self(3);
    pub const R4: Self = Self(4);
    pub const R5: Self = Self(5);
    pub const R6: Self = Self(6);
    pub const R7: Self = Self(7);
    pub const R8: Self = Self(8);
    pub const R9: Self = Self(9);
    pub const R10: Self = Self(10);
    pub const R11: Self = Self(11);
    pub const R12: Self = Self(12);
    pub const R13: Self = Self(13);
    pub const R14: Self = Self(14);
    pub const R15: Self = Self(15);
    pub const R16: Self = Self(16);
    pub const R17: Self = Self(17);
    pub const R18: Self = Self(18);
    pub const R19: Self = Self(19);
    pub const R20: Self = Self(20);
    pub const R21: Self = Self(21);
    pub const R22: Self = Self(22);
    pub const R23: Self = Self(23);
    pub const R24: Self = Self(24);
    pub const R25: Self = Self(25);
    pub const R26: Self = Self(26);
}
