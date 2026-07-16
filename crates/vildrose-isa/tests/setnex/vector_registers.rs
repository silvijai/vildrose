use vildrose_core::word::Tribble;
use vildrose_isa::setnex::csr::{CSR_COUNT, Csr};

#[test]
fn csr_count_is_twenty_seven() {
    assert_eq!(CSR_COUNT, 27);
}

#[test]
fn csr_indices_cover_the_whole_tribble_range() {
    assert_eq!(Csr::new(Tribble::from_int(-13).unwrap()).index(), 0);
    assert_eq!(Csr::new(Tribble::from_int(0).unwrap()).index(), 13);
    assert_eq!(Csr::new(Tribble::from_int(13).unwrap()).index(), 26);
}

#[test]
fn csr_from_int_accepts_only_tribble_values() {
    assert!(Csr::from_int(-13).is_some());
    assert!(Csr::from_int(0).is_some());
    assert!(Csr::from_int(13).is_some());
    assert!(Csr::from_int(-14).is_none());
    assert!(Csr::from_int(14).is_none());
}
