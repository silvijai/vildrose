use hyldeblomst::setnex::vector_registers::{SetnexVectorRegisters, VECTOR_LANE_COUNT, VectorWord};
use vildrose_core::trit::Trit;
use vildrose_isa::setnex::vector_registers::VectorRegister;

#[test]
fn all_setnex_vector_registers_start_as_zero() {
    let registers = SetnexVectorRegisters::new();
    let zero = [Trit::Z; VECTOR_LANE_COUNT];

    for register in VectorRegister::ALL {
        assert_eq!(
            registers.read(register),
            zero,
            "{register:?} should begin at zero",
        );
    }
}

#[test]
fn writing_and_reading_a_vector_register_round_trips() {
    let mut registers = SetnexVectorRegisters::new();
    let value: VectorWord = [
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
        Trit::N,
        Trit::Z,
        Trit::P,
    ];

    registers.write(VectorRegister::V5, value);

    assert_eq!(registers.read(VectorRegister::V5), value);
}

#[test]
fn writing_one_vector_register_does_not_change_another() {
    let mut registers = SetnexVectorRegisters::new();

    let first: VectorWord = [Trit::P; VECTOR_LANE_COUNT];
    let second: VectorWord = [Trit::N; VECTOR_LANE_COUNT];

    registers.write(VectorRegister::V1, first);
    registers.write(VectorRegister::V2, second);

    assert_eq!(registers.read(VectorRegister::V1), first);
    assert_eq!(registers.read(VectorRegister::V2), second);
}

#[test]
fn writing_a_vector_register_does_not_change_untouched_registers() {
    let mut registers = SetnexVectorRegisters::new();
    let zero = [Trit::Z; VECTOR_LANE_COUNT];
    let value = [Trit::P; VECTOR_LANE_COUNT];

    registers.write(VectorRegister::V7, value);

    assert_eq!(registers.read(VectorRegister::V7), value);
    assert_eq!(registers.read(VectorRegister::V6), zero);
    assert_eq!(registers.read(VectorRegister::V8), zero);
}
