use vildrose_core::trit::Trit;

#[test]
fn tmin_tmax_are_symmetric() {
    let trits = [Trit::N, Trit::Z, Trit::P];
    for a in trits {
        for b in trits {
            assert_eq!(a.tmin(b), b.tmin(a), "tmin not symmetric: {a} {b}");
            assert_eq!(a.tmax(b), b.tmax(a), "tmax not symmetric: {a} {b}");
        }
    }
}

#[test]
fn tmin_identity() {
    for t in [Trit::N, Trit::Z, Trit::P] {
        assert_eq!(t.tmin(t), t, "tmin(t, t) should equal t");
    }
}

#[test]
fn tmax_identity() {
    for t in [Trit::N, Trit::Z, Trit::P] {
        assert_eq!(t.tmax(t), t, "tmax(t, t) should equal t");
    }
}

#[test]
fn tnot_identity() {
    for t in [Trit::N, Trit::Z, Trit::P] {
        assert_eq!(t.tnot().tnot(), t, "tnot(tnot(t)) should equal t");
    }
}

#[test]
fn tmin_absorbing_n() {
    // N is absorbing element for tmin (Kleene AND)
    let n = Trit::N;
    for t in [Trit::N, Trit::Z, Trit::P] {
        assert_eq!(n.tmin(t), Trit::N, "N ∧ x should be N");
        assert_eq!(t.tmin(n), Trit::N, "x ∧ N should be N");
    }
}

#[test]
fn tmax_absorbing_p() {
    // P is absorbing element for tmax (Kleene OR)
    let p = Trit::P;
    for t in [Trit::N, Trit::Z, Trit::P] {
        assert_eq!(p.tmax(t), Trit::P, "P ∨ x should be P");
        assert_eq!(t.tmax(p), Trit::P, "x ∨ P should be P");
    }
}

#[test]
fn tnot_involution() {
    for t in [Trit::N, Trit::Z, Trit::P] {
        assert_eq!(t.tnot().tnot(), t, "¬(¬x) should be x");
    }
}

// <- Consensus
#[test]
fn consensus_with_z_is_z() {
    let z = Trit::Z;
    for t in [Trit::N, Trit::Z, Trit::P] {
        assert_eq!(z.consensus(t), Trit::Z, "consensus(Z, x) should be Z");
        assert_eq!(t.consensus(z), Trit::Z, "consensus(x, Z) should be Z");
    }
}

#[test]
fn consensus_same_value() {
    for t in [Trit::N, Trit::Z, Trit::P] {
        assert_eq!(t.consensus(t), t, "consensus(t, t) should be t");
    }
}

#[test]
fn consensus_opposite() {
    assert_eq!(
        Trit::N.consensus(Trit::P),
        Trit::Z,
        "consensus(N, P) should be Z"
    );
    assert_eq!(
        Trit::P.consensus(Trit::N),
        Trit::Z,
        "consensus(P, N) should be Z"
    );
}

#[test]
fn clip_is_identity() {
    for t in [Trit::N, Trit::Z, Trit::P] {
        assert_eq!(t.clip(), t, "clip should be identity");
    }
}

#[test]
fn sign_is_identity() {
    for t in [Trit::N, Trit::Z, Trit::P] {
        assert_eq!(t.sign(), t, "sign should be identity for trits");
    }
}
