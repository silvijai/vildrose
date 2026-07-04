use vildrose_core::word::{Tryte, Word27, Word54, WordType};

#[test]
fn tryte_roundtrip_positive_and_negative() {
    for n in [-9841i16, -100, -1, 0, 1, 100, 9841] {
        let t = Tryte::from_int(n).unwrap();
        assert_eq!(t.to_int(), n, "roundtrip failed for {n}");
    }
}

#[test]
fn tryte_from_int_rejects_out_of_range() {
    assert!(Tryte::from_int(Tryte::MAX_INT + 1).is_err());
    assert!(Tryte::from_int(Tryte::MIN_INT - 1).is_err());
}

#[test]
fn tryte_from_int_accepts_boundary_values() {
    assert!(Tryte::from_int(Tryte::MAX_INT).is_ok());
    assert!(Tryte::from_int(Tryte::MIN_INT).is_ok());
}

#[test]
fn tryte_min_max_int_are_symmetric() {
    assert_eq!(Tryte::MIN_INT, -Tryte::MAX_INT);
}

#[test]
fn word27_roundtrip_positive_and_negative() {
    for n in [Word27::MIN_INT, -1000, -1, 0, 1, 1000, Word27::MAX_INT] {
        let w = Word27::from_int(n).unwrap();
        assert_eq!(w.to_int(), n, "roundtrip failed for {n}");
    }
}

#[test]
fn word27_from_int_rejects_out_of_range() {
    assert!(Word27::from_int(Word27::MAX_INT + 1).is_err());
    assert!(Word27::from_int(Word27::MIN_INT - 1).is_err());
}

#[test]
fn word54_roundtrip_positive_and_negative() {
    for n in [Word54::MIN_INT, -1000, -1, 0, 1, 1000, Word54::MAX_INT] {
        let w = Word54::from_int(n).unwrap();
        assert_eq!(w.to_int(), n, "roundtrip failed for {n}");
    }
}

#[test]
fn word54_from_int_rejects_out_of_range() {
    assert!(Word54::from_int(Word54::MAX_INT + 1).is_err());
    assert!(Word54::from_int(Word54::MIN_INT - 1).is_err());
}

#[test]
fn from_trait_matches_to_int() {
    let t = Tryte::from_int(42).unwrap();
    let val: i16 = t.into();
    assert_eq!(val, t.to_int());
    assert_eq!(i16::from(t), t.to_int());
}

#[test]
fn try_from_matches_from_int() {
    assert_eq!(Tryte::try_from(42i16), Tryte::from_int(42));
    assert_eq!(
        Tryte::try_from(Tryte::MAX_INT + 1).is_err(),
        Tryte::from_int(Tryte::MAX_INT + 1).is_err()
    );
}

#[test]
fn zero_converts_to_zero_int() {
    assert_eq!(Tryte::zero().to_int(), 0);
    assert_eq!(Word27::zero().to_int(), 0);
    assert_eq!(Word54::zero().to_int(), 0);
}

#[test]
fn negative_values_convert_correctly() {
    // spot-check a value that requires a borrow during from_int's rem_euclid correction
    let t = Tryte::from_int(-5).unwrap();
    assert_eq!(t.to_int(), -5);
}
