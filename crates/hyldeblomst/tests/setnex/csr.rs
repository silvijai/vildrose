use hyldeblomst::setnex::csr::SetnexCsrs;
use vildrose_core::word::Word27;
use vildrose_isa::setnex::csr::Csr;

#[test]
fn all_setnex_csrs_start_as_zero() {
    let csrs = SetnexCsrs::new();

    for value in -13..=13 {
        let csr = Csr::from_int(value).unwrap();
        assert_eq!(
            csrs.read(csr),
            Word27::zero(),
            "{csr:?} should begin at zero",
        );
    }
}

#[test]
fn writing_and_reading_a_csr_round_trips() {
    let mut csrs = SetnexCsrs::new();
    let csr = Csr::from_int(0).unwrap();
    let value = Word27::from_int(42).unwrap();

    csrs.write(csr, value);

    assert_eq!(csrs.read(csr), value);
}

#[test]
fn writing_one_csr_does_not_change_another() {
    let mut csrs = SetnexCsrs::new();

    let first = Csr::from_int(-1).unwrap();
    let second = Csr::from_int(1).unwrap();

    csrs.write(first, Word27::from_int(10).unwrap());
    csrs.write(second, Word27::from_int(-4).unwrap());

    assert_eq!(csrs.read(first).to_int(), 10);
    assert_eq!(csrs.read(second).to_int(), -4);
}
