use hyldeblomst::setnex::registers::SetnexRegisters;
use vildrose_core::word::Word27;
use vildrose_isa::setnex::registers::Register;

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
