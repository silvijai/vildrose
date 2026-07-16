use vildrose_core::word::Tribble;
use vildrose_isa::setnex::csr::{CSR_COUNT, Csr};

#[test]
fn setnex_has_twenty_seven_csrs() {
    assert_eq!(CSR_COUNT, 27);
}

#[test]
fn valid_csr_addresses_construct_csrs() {
    for value in -13..=13 {
        let csr = Csr::from_int(value).expect("tribble-range value should construct CSR");
        assert_eq!(csr.address(), Tribble::from_int(value).unwrap());
    }
}

#[test]
fn invalid_csr_addresses_are_rejected() {
    assert_eq!(Csr::from_int(-14), None);
    assert_eq!(Csr::from_int(14), None);
    assert_eq!(Csr::from_int(i8::MIN), None);
    assert_eq!(Csr::from_int(i8::MAX), None);
}

#[test]
fn csr_indices_cover_the_whole_tribble_range() {
    assert_eq!(Csr::from_int(-13).unwrap().index(), 0);
    assert_eq!(Csr::from_int(0).unwrap().index(), 13);
    assert_eq!(Csr::from_int(13).unwrap().index(), 26);
}

#[test]
fn csr_new_preserves_the_tribble_address() {
    let address = Tribble::from_int(5).unwrap();
    let csr = Csr::new(address);

    assert_eq!(csr.address(), address);
}

#[test]
fn csr_constants_match_spec_addresses() {
    assert_eq!(Csr::PC.address().to_int(), 1);
    assert_eq!(Csr::LMODE.address().to_int(), 2);
    assert_eq!(Csr::FLAGS.address().to_int(), 3);
    assert_eq!(Csr::ETVAL2.address().to_int(), 13);
    assert_eq!(Csr::MPU_SELECT.address().to_int(), -1);
    assert_eq!(Csr::IPRIORITY.address().to_int(), -6);
}
