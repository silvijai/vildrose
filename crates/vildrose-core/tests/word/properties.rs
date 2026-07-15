use crate::common::tryte_strategy;
use proptest::prelude::*;
use vildrose_core::word::{Tryte, Word27};

proptest! {
    // Commutativity
    #[test]
    fn add_is_commutative(a in tryte_strategy(), b in tryte_strategy()) {
        prop_assert_eq!(a + b, b + a);
    }

    #[test]
    fn mul_is_commutative(a in tryte_strategy(), b in tryte_strategy()) {
        prop_assert_eq!(a * b, b * a);
    }

    // Identities and annihilators
    #[test]
    fn add_zero_is_identity(t in tryte_strategy()) {
        prop_assert_eq!(t + Tryte::zero(), t);
        prop_assert_eq!(Tryte::zero() + t, t);
    }

    #[test]
    fn mul_one_is_identity(t in tryte_strategy()) {
        let one = Tryte::from_int(1).expect("1 must fit in Tryte");
        prop_assert_eq!(t * one, t);
        prop_assert_eq!(one * t, t);
    }

    #[test]
    fn mul_zero_annihilates(t in tryte_strategy()) {
        prop_assert_eq!((t * Tryte::zero()).to_int(), 0);
        prop_assert_eq!((Tryte::zero() * t).to_int(), 0);
    }

    // Negation and subtraction
    #[test]
    fn neg_is_involution(t in tryte_strategy()) {
        prop_assert_eq!(-(-t), t);
    }

    #[test]
    fn sub_self_is_zero(t in tryte_strategy()) {
        prop_assert_eq!((t - t).to_int(), 0);
    }

    #[test]
    fn zero_minus_self_equals_negate(t in tryte_strategy()) {
        prop_assert_eq!(Tryte::zero() - t, -t);
    }

    // Ordering and sign
    #[test]
    fn sign_matches_int_sign(t in tryte_strategy()) {
        let n = t.to_int();

        let sign = i16::from(t.sign().value());
        prop_assert_eq!(sign > 0, n > 0);
        prop_assert_eq!(sign < 0, n < 0);
    }

    #[test]
    fn abs_is_never_negative(t in tryte_strategy()) {
        prop_assert!(t.abs().to_int() >= 0);
    }

    #[test]
    fn abs_matches_native_abs_within_range(a in tryte_strategy()) {
        // Range is symmetric, so -MIN_INT == MAX_INT and this is safe.
        prop_assert_eq!(a.abs().to_int(), a.to_int().abs());
    }

    // Cross-width consistency
    #[test]
    fn cross_width_ord_consistent_with_int(n in -1000i16..=1000i16) {
        let narrow = Tryte::from_int(n).expect("n must fit Tryte");
        let wide   = Word27::from_int(i64::from(n)).expect("n must fit Word27");

        prop_assert_eq!(i64::from(narrow.to_int()), wide.to_int());
    }
}
