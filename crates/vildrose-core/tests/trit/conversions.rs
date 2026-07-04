use vildrose_core::trit::Trit;

#[test]
fn display_nzp() {
    assert_eq!(Trit::N.to_string(), "N");
    assert_eq!(Trit::Z.to_string(), "Z");
    assert_eq!(Trit::P.to_string(), "P");
}

#[test]
fn try_from_i8_valid() {
    assert_eq!(Trit::try_from(-1i8), Ok(Trit::N));
    assert_eq!(Trit::try_from(0i8), Ok(Trit::Z));
    assert_eq!(Trit::try_from(1i8), Ok(Trit::P));
}

#[test]
fn try_from_i8_invalid() {
    assert!(Trit::try_from(2i8).is_err());
    assert!(Trit::try_from(-2i8).is_err());
    assert!(Trit::try_from(127i8).is_err());
}

#[test]
fn from_i8_coercion() {
    assert_eq!(i8::from(Trit::N), -1);
    assert_eq!(i8::from(Trit::Z), 0);
    assert_eq!(i8::from(Trit::P), 1);
}

#[test]
fn is_zero() {
    assert!(Trit::Z.is_zero());
    assert!(!Trit::N.is_zero());
    assert!(!Trit::P.is_zero());
}

#[test]
fn is_positive() {
    assert!(Trit::P.is_positive());
    assert!(!Trit::N.is_positive());
    assert!(!Trit::Z.is_positive());
}

#[test]
fn is_negative() {
    assert!(Trit::N.is_negative());
    assert!(!Trit::P.is_negative());
    assert!(!Trit::Z.is_negative());
}
