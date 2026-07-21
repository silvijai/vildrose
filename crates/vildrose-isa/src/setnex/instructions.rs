//! Setnex instruction set.

// TODO: add a dedicated 4-trit opcode field type in vildrose_core or setnex encoding helpers

use crate::setnex::registers::Register;
use vildrose_core::word::Word27;

/// One raw Setnex instruction word (27 trits).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawInstruction(pub Word27);

/// Per-spec primary opcode (t[0..=3]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryOpcode {
    // <- AlU group - R format
    /// Add family (basic addition, saturating and carry)
    AddFamily = -40,
    /// Subtract family (basic subtraction, saturating and borrow)
    SubFamily = -39,
    /// Multiply family (basic multiplaction and high mode (54-trit word conversion))
    MulFamily = -38,
    /// Division instruction
    Div = -37,
    /// Modulo instruction
    Mod = -36,
    /// Negate instruction
    Neg = -35,
    /// Bitwise AND instruction
    Tand = -34,
    /// Bitwise OR instruction
    Tor = -33,
    /// Bitwise NOT instruction
    Tnot = -32,
    /// Bitwise IMPL instruction
    TImpl = -31,
    /// Cons instruction
    Cons = -30,
    /// Acons instruction
    Acons = -29,
    /// Trit shift instruction
    TShift = -28,
    /// Trit-by-trit compare instruction
    TCMP = -27,

    // <- Memory group - I format
    /// Load from register
    Load = -26,
    /// Store to register
    Store = -25,
    /// Load immediate instruction
    LI = -24,
    /// Load upper immediate instruction
    LUI = -23,
    /// Add immediate instruction
    ADDI = -22,
    /// Branch on true instruction
    BRT3 = -21,
    // Opcode -20 to -19 reserved
    /// Compare immediate instruction
    CMPI = -18,

    // <- Branch group - J and U format
    /// Branch on equal instruction
    BEQ = -17,
    /// Branch on not equal instruction
    BNE = -16,
    /// Branch on less than instruction
    BLT = -15,
    /// Branch on greater than instruction
    BGT = -14,
    /// Branch on less than or equal instruction
    BLE = -13,
    /// Branch on greater than or equal instruction
    BGE = -12,
    /// Jump to address instruction
    JMPA = -11,
    /// Branch on false instruction
    BF = -10,
    /// Jump instruction
    JMP = -9,
    /// Call instruction
    CALL = -8,

    // <- CSR and system group - I format
    /// Read CSR instruction
    CSRR = -7,
    /// Write CSR instruction
    CSRW = -6,
    /// Atomic read-then-write CSR instruction
    CSRX = -5,
    /// IDK yet this is a bit complex...
    ECALL = -4,

    // <- Special group - R format
    // I still need to understand this better
    /// Atomic exception return instruction
    IRET = -3,
    /// Register select instruction
    TSEL = -2,
    /// No-op instruction
    NOP = -1,
    /// Halt processor instruction
    HALT = 0,
    /// Get trit instruction
    TGET = 1,
    /// Set trit instruction
    TSET = 2, // (funct[0] for defining value)
    /// Sign trit instruction
    TSIGN = 3,
    /// Compare trit instruction
    CMP = 4,

    // <- Absolute value and trit-reduce
    /// Absolute value instruction
    TABS = 5,
    /// Minimum trit instruction
    TMIN = 6,
    /// Maximum trit instruction
    TMAX = 7,

    // <- TFP group - R format
    /// Floating-point addition instruction
    FADD = 8,
    /// Floating-point subtraction instruction
    FSUB = 9,
    /// Floating-point multiplication instruction
    FMUL = 10,
    /// Floating-point division instruction
    FDIV = 11,
    /// Floating-point compare instruction
    FCMP = 12, // returns flags
    /// Floating-point conversion instruction
    FCVT = 13, // (funct[0] for rounding / conversion mode)
    // 14 is reserved

    // <- Vector group - R format
    /// Vector lane-wise add/subtraction family
    VADDSUBFamily = 15,
    /// Vector lane-wise multiplication
    VMUL = 16,
    /// Vector log family
    VLOG = 17,
    /// Vector lane-wise select family
    VSEL = 18,
    /// Vector lane-wise compare family
    VCMP = 19,
    /// Vector lane-wise reduction family
    VRED = 20,
    /// Vector lane-wise permutation family
    VPERM = 21,
    /// Inter- and intra-bank movement
    VMOVE = 22,
    // 23-40 are reserved
}

/// Instruction format (t[4..=6]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// Register-register format
    R,
    /// Immediate format
    I,
    /// Conditional branch format
    J,
    /// Unconditional jump format
    U,
    /// Ternary three-way branch format
    B,
}

/// Common three-register payload used by simple arithmetic instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reg3 {
    /// Destination register that receives the result.
    pub rd: Register,
    /// First source register.
    pub rs1: Register,
    /// Second source register.
    pub rs2: Register,
}

/// Add-with-carry instruction payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdcInstr {
    /// Destination register that receives the sum.
    pub rd: Register,
    /// First source register.
    pub rs1: Register,
    /// Second source register.
    pub rs2: Register,
    // TODO: Set carry flag.
}

/// Subtract-with-borrow instruction payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SbcInstr {
    /// Destination register that receives the subtraction result.
    pub rd: Register,
    /// Minuend source register.
    pub rs1: Register,
    /// Subtrahend source register.
    pub rs2: Register,
    // TODO: Set borrow flag.
}

#[allow(missing_docs)]
/// Fully decoded, semantic Setnex instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodedInstr {
    /// Addition instruction.
    ///
    /// Adds the values of `rs1` and `rs2`, storing the result in `rd`.
    Add(Reg3),

    /// Saturating addition instruction.
    ///
    /// Adds the values of `rs1` and `rs2`, storing the result in `rd`,
    /// and saturates the result to the destination range.
    Adds(Reg3),

    /// Add with carry instruction.
    ///
    /// Adds the values of `rs1` and `rs2`, storing the result in `rd`,
    /// and updates carry-related state.
    Adc(AdcInstr),

    /// Subtraction instruction.
    ///
    /// Subtracts the value of `rs2` from `rs1`, storing the result in `rd`.
    Sub(Reg3),

    /// Saturating subtraction instruction.
    ///
    /// Subtracts the value of `rs2` from `rs1`, storing the result in `rd`,
    /// and saturates the result to the destination range.
    Subs(Reg3),

    /// Subtract with borrow instruction.
    ///
    /// Subtracts the value of `rs2` from `rs1`, storing the result in `rd`,
    /// and updates borrow-related state.
    Sbc(SbcInstr),
    // TODO: Set this up for the rest of the instructions
}
