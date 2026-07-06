use vildrose_core::word::{CheckedDiv, Tryte, Word27, Word54};

#[test]
fn tryte_div_exact() {
    let a = Tryte::from_int(12).unwrap();
    let b = Tryte::from_int(4).unwrap();
    assert_eq!((a / b).to_int(), 3);
}

#[test]
fn tryte_div_truncates_toward_zero() {
    let a = Tryte::from_int(7).unwrap();
    let b = Tryte::from_int(2).unwrap();
    assert_eq!((a / b).to_int(), 3);

    let neg = Tryte::from_int(-7).unwrap();
    assert_eq!(
        (neg / b).to_int(),
        -3,
        "division should truncate toward zero, not floor"
    );
}

#[test]
fn tryte_div_negative_divisor() {
    let a = Tryte::from_int(12).unwrap();
    let b = Tryte::from_int(-4).unwrap();
    assert_eq!((a / b).to_int(), -3);
}

#[test]
#[should_panic(expected = "division by zero")]
fn tryte_div_by_zero_panics() {
    let a = Tryte::from_int(5).unwrap();
    let _ = a / Tryte::zero();
}

#[test]
fn tryte_checked_div_by_zero_returns_none() {
    let a = Tryte::from_int(5).unwrap();
    assert_eq!(a.checked_div(Tryte::zero()), None);
}

#[test]
fn tryte_checked_div_normal_case_returns_some() {
    let a = Tryte::from_int(9).unwrap();
    let b = Tryte::from_int(3).unwrap();
    assert_eq!(a.checked_div(b).unwrap().to_int(), 3);
}

#[test]
fn tryte_checked_div_min_by_negative_one_is_symmetric() {
    let min = Tryte::from_int(Tryte::MIN_INT).unwrap();
    let neg_one = Tryte::from_int(-1).unwrap();
    assert_eq!(min.checked_div(neg_one).unwrap().to_int(), Tryte::MAX_INT);
}

#[test]
fn tryte_div_min_by_negative_one_does_not_panic() {
    let min = Tryte::from_int(Tryte::MIN_INT).unwrap();
    let neg_one = Tryte::from_int(-1).unwrap();
    assert_eq!((min / neg_one).to_int(), Tryte::MAX_INT);
}

#[test]
fn word27_div_exact() {
    let a = Word27::from_int(1000).unwrap();
    let b = Word27::from_int(25).unwrap();
    assert_eq!((a / b).to_int(), 40);
}

#[test]
fn tryte_div_word27_cross_width() {
    let t = Tryte::from_int(20).unwrap();
    let w = Word27::from_int(4).unwrap();
    assert_eq!((t / w).to_int(), 5);
}

#[test]
fn word27_div_tryte_cross_width() {
    let w = Word27::from_int(20).unwrap();
    let t = Tryte::from_int(4).unwrap();
    assert_eq!((w / t).to_int(), 5);
}

#[test]
fn word54_checked_div_cross_width_by_zero() {
    let w = Word54::from_int(20).unwrap();
    let t = Tryte::zero();
    assert_eq!(w.checked_div(t), None);
}
