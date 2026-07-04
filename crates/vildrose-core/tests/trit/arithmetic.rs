use vildrose_core::trit::Trit;

#[test]
fn negate_is_involution() {
    for t in [Trit::N, Trit::Z, Trit::P] {
        assert_eq!(t.negate().negate(), t);
    }
}

#[test]
fn add_exhaustive() {
    type Case = ((Trit, Trit), (Trit, Trit));
    let cases: &[Case] = &[
        ((Trit::N, Trit::N), (Trit::P, Trit::N)),
        ((Trit::N, Trit::Z), (Trit::N, Trit::Z)),
        ((Trit::N, Trit::P), (Trit::Z, Trit::Z)),
        ((Trit::Z, Trit::N), (Trit::N, Trit::Z)),
        ((Trit::Z, Trit::Z), (Trit::Z, Trit::Z)),
        ((Trit::Z, Trit::P), (Trit::P, Trit::Z)),
        ((Trit::P, Trit::N), (Trit::Z, Trit::Z)),
        ((Trit::P, Trit::Z), (Trit::P, Trit::Z)),
        ((Trit::P, Trit::P), (Trit::N, Trit::P)),
    ];
    for &((a, b), expected) in cases {
        assert_eq!(a.add(b), expected, "add({a}, {b})");
    }
}

#[test]
fn add_zero_identity() {
    for t in [Trit::N, Trit::Z, Trit::P] {
        let (sum, carry) = t.add(Trit::Z);
        assert_eq!(sum, t, "t + 0 sum should be t");
        assert_eq!(carry, Trit::Z, "t + 0 carry should be 0");
    }
}

#[test]
fn add_commutative() {
    let trits = [Trit::N, Trit::Z, Trit::P];
    for a in trits {
        for b in trits {
            assert_eq!(
                a.add(b),
                b.add(a),
                "add({a}, {b}) should equal add({b}, {a})"
            );
        }
    }
}

#[test]
fn add_n_n_produces_carry() {
    let (sum, carry) = Trit::N.add(Trit::N);
    assert_eq!(
        sum,
        Trit::P,
        "(-1) + (-1) sum should be 1 (wraps to 1 from -2)"
    );
    assert_eq!(carry, Trit::N, "(-1) + (-1) carry should be -1");
}

#[test]
fn add_p_p_produces_carry() {
    let (sum, carry) = Trit::P.add(Trit::P);
    assert_eq!(sum, Trit::N, "1 + 1 sum should be -1 (wraps to -1 from 2)");
    assert_eq!(carry, Trit::P, "1 + 1 carry should be 1");
}

// <- Absolute value tests
#[test]
fn abs_is_idempotent() {
    for t in [Trit::N, Trit::Z, Trit::P] {
        assert_eq!(t.abs().abs(), t.abs(), "abs(abs(t)) == abs(t)");
    }
}

#[test]
fn abs_positive_unchanged() {
    assert_eq!(Trit::Z.abs(), Trit::Z);
    assert_eq!(Trit::P.abs(), Trit::P);
}

#[test]
fn abs_negative_becomes_positive() {
    assert_eq!(Trit::N.abs(), Trit::P);
}

#[test]
fn inc_exhaustive() {
    let cases = [(Trit::N, Trit::Z), (Trit::Z, Trit::P), (Trit::P, Trit::N)];
    for &(input, expected) in &cases {
        assert_eq!(input.inc(), expected, "inc({input}) should be {expected}");
    }
}

#[test]
fn dec_exhaustive() {
    let cases = [(Trit::N, Trit::P), (Trit::Z, Trit::N), (Trit::P, Trit::Z)];
    for &(input, expected) in &cases {
        assert_eq!(input.dec(), expected, "dec({input}) should be {expected}");
    }
}

#[test]
fn increment_and_decrement_are_inverses() {
    for t in [Trit::N, Trit::Z, Trit::P] {
        assert_eq!(t.inc().dec(), t, "inc(dec(t)) == t");
        assert_eq!(t.dec().inc(), t, "dec(inc(t)) == t");
    }
}
