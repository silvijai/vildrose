use hyldeblomst::setnex::registers::SetnexRegisters;
use vildrose_core::word::Word27;
use vildrose_isa::setnex::registers::{REGISTER_COUNT, Register};

#[test]
fn setnex_has_twenty_seven_registers() {
    assert_eq!(REGISTER_COUNT, 27);
}

#[test]
fn valid_register_numbers_construct_registers() {
    for register in Register::ALL {
        let number = register.number();

        assert_eq!(Register::new(number), Some(register));
    }
}

#[test]
fn invalid_register_numbers_are_rejected() {
    assert_eq!(Register::new(27), None);
    assert_eq!(Register::new(u8::MAX), None);
}

#[test]
fn named_registers_have_expected_numbers() {
    assert_eq!(Register::R0.number(), 0);
    assert_eq!(Register::R1.number(), 1);
    assert_eq!(Register::R13.number(), 13);
    assert_eq!(Register::R14.number(), 14);
    assert_eq!(Register::R26.number(), 26);
}

#[test]
fn abi_aliases_name_the_expected_physical_registers() {
    assert_eq!(Register::ZERO, Register::R0);
    assert_eq!(Register::RA, Register::R1);
    assert_eq!(Register::SP, Register::R2);
    assert_eq!(Register::T0, Register::R5);
    assert_eq!(Register::S0, Register::R8);
    assert_eq!(Register::A0, Register::R10);
    assert_eq!(Register::A7, Register::R17);
    assert_eq!(Register::T3, Register::R26);
}

#[test]
fn all_setnex_registers_start_as_zero() {
    let registers = SetnexRegisters::new();

    for register in Register::ALL {
        assert_eq!(
            registers.read(register),
            Word27::zero(),
            "{register:?} should begin at zero",
        );
    }
}

#[test]
fn writing_and_reading_a_normal_register_round_trips() {
    let mut registers = SetnexRegisters::new();
    let value = Word27::from_int(42).unwrap();

    registers.write(Register::R5, value);

    assert_eq!(registers.read(Register::R5), value);
}

#[test]
fn writing_one_register_does_not_change_another() {
    let mut registers = SetnexRegisters::new();

    registers.write(Register::R1, Word27::from_int(10).unwrap());
    registers.write(Register::R2, Word27::from_int(-4).unwrap());

    assert_eq!(registers.read(Register::R1).to_int(), 10);
    assert_eq!(registers.read(Register::R2).to_int(), -4);
}

#[test]
fn r0_is_hardwired_to_zero() {
    let mut registers = SetnexRegisters::new();

    registers.write(Register::R0, Word27::from_int(123).unwrap());

    assert_eq!(registers.read(Register::R0), Word27::zero());
}

#[test]
fn writing_r0_does_not_affect_other_registers() {
    let mut registers = SetnexRegisters::new();

    registers.write(Register::R1, Word27::from_int(17).unwrap());
    registers.write(Register::R0, Word27::from_int(-99).unwrap());

    assert_eq!(registers.read(Register::R1).to_int(), 17);
    assert_eq!(registers.read(Register::R0), Word27::zero());
}
