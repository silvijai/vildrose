#![allow(missing_docs)]

use vildrose_core::kleene::Kleene;

const ALL: [Kleene; 3] = [Kleene::FALSE, Kleene::UNKNOWN, Kleene::TRUE];

#[test]
fn not_truth_table() {
    assert_eq!(!Kleene::FALSE, Kleene::TRUE);
    assert_eq!(!Kleene::UNKNOWN, Kleene::UNKNOWN);
    assert_eq!(!Kleene::TRUE, Kleene::FALSE);
}

#[test]
fn double_negation_is_involution() {
    for &a in &ALL {
        assert_eq!(!!a, a);
    }
}

#[test]
fn and_is_commutative() {
    for &a in &ALL {
        for &b in &ALL {
            assert_eq!(a.and(b), b.and(a));
        }
    }
}

#[test]
fn or_is_commutative() {
    for &a in &ALL {
        for &b in &ALL {
            assert_eq!(a.or(b), b.or(a));
        }
    }
}

#[test]
fn and_or_absorb_identities() {
    for &a in &ALL {
        assert_eq!(a.and(Kleene::TRUE), a);
        assert_eq!(a.or(Kleene::FALSE), a);
        assert_eq!(a.and(Kleene::FALSE), Kleene::FALSE);
        assert_eq!(a.or(Kleene::TRUE), Kleene::TRUE);
    }
}

#[test]
fn unknown_is_absorbing_for_and_when_other_is_unknown_or_true() {
    assert_eq!(Kleene::UNKNOWN.and(Kleene::TRUE), Kleene::UNKNOWN);
    assert_eq!(Kleene::UNKNOWN.and(Kleene::UNKNOWN), Kleene::UNKNOWN);
    // The one case UNKNOWN does NOT dominate: AND with FALSE is still FALSE.
    assert_eq!(Kleene::UNKNOWN.and(Kleene::FALSE), Kleene::FALSE);
}

#[test]
fn de_morgans_laws_hold() {
    for &a in &ALL {
        for &b in &ALL {
            assert_eq!(!(a.and(b)), (!a).or(!b));
            assert_eq!(!(a.or(b)), (!a).and(!b));
        }
    }
}

#[test]
fn implies_matches_material_definition() {
    for &a in &ALL {
        for &b in &ALL {
            assert_eq!(a.implies(b), (!a).or(b));
        }
    }
}

#[test]
fn true_implies_true_and_false_implies_anything() {
    assert_eq!(Kleene::FALSE.implies(Kleene::FALSE), Kleene::TRUE);
    assert_eq!(Kleene::FALSE.implies(Kleene::TRUE), Kleene::TRUE);
    assert_eq!(Kleene::TRUE.implies(Kleene::FALSE), Kleene::FALSE);
    assert_eq!(Kleene::TRUE.implies(Kleene::TRUE), Kleene::TRUE);
}

#[test]
fn iff_is_reflexive_for_definite_values() {
    // Only TRUE and FALSE are reflexive under iff. This is because Kleene
    // logic is purely truth-functional and has no notion of syntactic
    // identity between operands — UNKNOWN.iff(UNKNOWN) evaluates to
    // UNKNOWN, not TRUE, since "if unknown then unknown" is itself unknown.
    assert_eq!(Kleene::TRUE.iff(Kleene::TRUE), Kleene::TRUE);
    assert_eq!(Kleene::FALSE.iff(Kleene::FALSE), Kleene::TRUE);
}

#[test]
fn unknown_iff_unknown_is_unknown() {
    // This is the well-known quirk of Kleene's K3: self-implication
    // is not guaranteed true when the value is UNKNOWN, because the
    // logic evaluates purely on truth values, not propositional identity.
    assert_eq!(Kleene::UNKNOWN.iff(Kleene::UNKNOWN), Kleene::UNKNOWN);
}

#[test]
fn iff_is_symmetric() {
    for &a in &ALL {
        for &b in &ALL {
            assert_eq!(a.iff(b), b.iff(a));
        }
    }
}

#[test]
fn query_helpers_match_constants() {
    assert!(Kleene::TRUE.is_true());
    assert!(Kleene::FALSE.is_false());
    assert!(Kleene::UNKNOWN.is_unknown());
    assert!(!Kleene::TRUE.is_false());
    assert!(!Kleene::UNKNOWN.is_true());
}

#[test]
fn ord_matches_trit_ord() {
    assert!(Kleene::FALSE < Kleene::UNKNOWN);
    assert!(Kleene::UNKNOWN < Kleene::TRUE);
}
