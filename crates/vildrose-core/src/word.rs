//! Fixed-width balanced ternary word types.
use crate::trit::Trit;

// <- Macros defined here
macro_rules! define_word {
    (
        $( #[$meta:meta] )*
        $name:ident($width:literal)
    ) => {
        $( #[$meta] )*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        /// A fixed-width balanced ternary word type with a width of $width trits.
        pub struct $name([Trit; $width]);

        impl_word_methods!($name, $width);
    };
}

macro_rules! impl_word_methods {
    ($name:ident, $width:literal) => {
        impl $name {
            /// Creates a new word filled with zeros
            pub const fn new(t: [Trit; $width]) -> Self {
                Self(t)
            }

            /// Creates a new word from an array of trits
            pub const fn from_trits(trits: [Trit; $width]) -> Self {
                Self(trits)
            }

            /// Creates a new word filled with zeros
            pub const fn zero() -> Self {
                Self([Trit::Z; $width])
            }

            /// Returns the trit at the given index, panicking if the index is out of bounds.
            pub fn trit(&self, i: usize) -> Trit {
                if i < $width {
                    self.0[i]
                } else {
                    panic!("Index out of bounds for {}: {}", stringify!($name), i);
                }
            }

            /// Returns the inverted (negated) form of the word
            #[must_use]
            pub fn negate(&self) -> Self {
                Self(self.0.map(|t| t.negate()))
            }

            /// Returns the absolute (no negatives) form of the word
            #[must_use]
            pub fn abs(&self) -> Self {
                if self.sign() == Trit::N {
                    self.negate()
                } else {
                    *self
                }
            }

            /// Returns the sign (whether it's negative, positive or zero) of the word
            #[must_use]
            pub fn sign(&self) -> Trit {
                for t in self.0.iter().rev() {
                    if *t != Trit::Z {
                        return *t;
                    }
                }
                Trit::Z
            }

            /// Returns the minimum of two words, element-wise
            #[must_use]
            pub fn tmin(&self, rhs: Self) -> Self {
                Self(std::array::from_fn(|i| self.0[i].tmin(rhs.0[i])))
            }

            /// Returns the maximum of two words, element-wise
            #[must_use]
            pub fn tmax(&self, rhs: Self) -> Self {
                Self(std::array::from_fn(|i| self.0[i].tmax(rhs.0[i])))
            }

            /// Returns the tritwise negation of the word
            #[must_use]
            pub fn tnot(&self) -> Self {
                Self(self.0.map(|t| t.negate()))
            }

            /// Returns the tritwise clipping of the word
            #[must_use]
            pub fn tclip(&self) -> Self {
                Self(self.0.map(|t| t.clip()))
            }

            /// Returns the tritwise sign of the word
            #[must_use]
            pub fn tsign(&self) -> Self {
                let s = self.sign();
                Self(std::array::from_fn(|i| if i == 0 { s } else { Trit::Z }))
            }

            /// Returns the tritwise consensus of two words
            #[must_use]
            pub fn tconsensus(&self, rhs: Self) -> Self {
                Self(std::array::from_fn(|i| self.0[i].consensus(rhs.0[i])))
            }

            /// Trit shift left by n positions, filling with zeros
            #[must_use]
            pub fn tshl(&self, n: usize) -> Self {
                Self(std::array::from_fn(|i| {
                    if i < n { Trit::Z } else { self.0[i - n] }
                }))
            }

            /// Trit shift right by n positions, filling with the sign of the word
            #[must_use]
            pub fn tshr(&self, n: usize) -> Self {
                let sign = self.sign();
                Self(std::array::from_fn(|i| {
                    if i + n < $width { self.0[i + n] } else { sign }
                }))
            }

            /// Trit logical shift right by n positions, filling with zeros
            #[must_use]
            pub fn tlshr(&self, n: usize) -> Self {
                Self(std::array::from_fn(|i| {
                    if i + n < $width {
                        self.0[i + n]
                    } else {
                        Trit::Z
                    }
                }))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for t in self.0.iter() {
                    write!(f, "{}", t)?;
                }
                Ok(())
            }
        }
    };
}

#[expect(dead_code)]
trait WordType: Sized {
    type Int;

    const MIN: Self::Int;
    const MAX: Self::Int;

    fn to_int(self) -> Self::Int;
    fn from_int(val: Self::Int) -> Result<Self, &'static str>;
}

// <- Definitions
define_word! {
    Tryte(9)
}

define_word! {
    Word27(27)
}

/// Word9 definition, it's just an alias for Tryte
pub type Word9 = Tryte;

// <- Tests (TODO: Rewrite these myself)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tryte_new_initializes_with_provided_array() {
        let trits = [
            Trit::N,
            Trit::Z,
            Trit::P,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ];
        let t = Tryte::new(trits);
        assert_eq!(t.trit(0), Trit::N);
        assert_eq!(t.trit(1), Trit::Z);
        assert_eq!(t.trit(2), Trit::P);
        assert_eq!(t.trit(3), Trit::Z);
        assert_eq!(t.trit(4), Trit::Z);
        assert_eq!(t.trit(5), Trit::Z);
        assert_eq!(t.trit(6), Trit::Z);
        assert_eq!(t.trit(7), Trit::Z);
        assert_eq!(t.trit(8), Trit::Z);
    }

    #[test]
    fn tryte_zero() {
        let t = Tryte::zero();
        for i in 0..9 {
            assert_eq!(t.trit(i), Trit::Z);
        }
    }

    #[test]
    fn tryte_from_trits() {
        let trits = [Trit::P; 9];
        let t = Tryte::from_trits(trits);
        for i in 0..9 {
            assert_eq!(t.trit(i), Trit::P);
        }
    }

    #[test]
    fn tryte_trit_access() {
        let trits = [
            Trit::N,
            Trit::Z,
            Trit::P,
            Trit::N,
            Trit::Z,
            Trit::P,
            Trit::N,
            Trit::Z,
            Trit::P,
        ];
        let t = Tryte::new(trits);
        for (i, trit) in trits.iter().enumerate() {
            assert_eq!(t.trit(i), *trit);
        }
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn tryte_trit_out_of_bounds() {
        let t = Tryte::zero();
        let _ = t.trit(9);
    }

    #[test]
    fn tryte_negate() {
        let trits = [
            Trit::N,
            Trit::Z,
            Trit::P,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ];
        let t = Tryte::new(trits);
        let neg = t.negate();
        assert_eq!(neg.trit(0), Trit::P);
        assert_eq!(neg.trit(1), Trit::Z);
        assert_eq!(neg.trit(2), Trit::N);
        assert_eq!(neg.trit(3), Trit::Z);
        assert_eq!(neg.trit(4), Trit::Z);
        assert_eq!(neg.trit(5), Trit::Z);
        assert_eq!(neg.trit(6), Trit::Z);
        assert_eq!(neg.trit(7), Trit::Z);
        assert_eq!(neg.trit(8), Trit::Z);
    }

    #[test]
    fn tryte_negate_involution() {
        let t = Tryte::new([
            Trit::P,
            Trit::N,
            Trit::Z,
            Trit::P,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ]);
        assert_eq!(t.negate().negate(), t);
    }

    #[test]
    fn tryte_abs_positive() {
        let t = Tryte::new([
            Trit::P,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ]);
        assert_eq!(t.abs(), t);
    }

    #[test]
    fn tryte_abs_negative() {
        let t = Tryte::new([
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::N,
        ]);
        let abs = t.abs();
        assert_eq!(abs.trit(8), Trit::P);
    }

    #[test]
    fn tryte_sign_positive() {
        let t = Tryte::new([
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::P,
        ]);
        assert_eq!(t.sign(), Trit::P);
    }

    #[test]
    fn tryte_sign_negative() {
        let t = Tryte::new([
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::N,
        ]);
        assert_eq!(t.sign(), Trit::N);
    }

    #[test]
    fn tryte_sign_zero() {
        let t = Tryte::zero();
        assert_eq!(t.sign(), Trit::Z);
    }

    #[test]
    fn tryte_tmin() {
        let t1 = Tryte::new([
            Trit::N,
            Trit::P,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ]);
        let t2 = Tryte::new([
            Trit::P,
            Trit::N,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ]);
        let min = t1.tmin(t2);
        assert_eq!(min.trit(0), Trit::N); // N ∧ P = N
        assert_eq!(min.trit(1), Trit::N); // P ∧ N = N
    }

    #[test]
    fn tryte_tmax() {
        let t1 = Tryte::new([
            Trit::N,
            Trit::P,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ]);
        let t2 = Tryte::new([
            Trit::P,
            Trit::N,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ]);
        let max = t1.tmax(t2);
        assert_eq!(max.trit(0), Trit::P); // N ∨ P = P
        assert_eq!(max.trit(1), Trit::P); // P ∨ N = P
    }

    #[test]
    fn tryte_tnot() {
        let t = Tryte::new([
            Trit::N,
            Trit::Z,
            Trit::P,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ]);
        let not = t.tnot();
        assert_eq!(not.trit(0), Trit::P);
        assert_eq!(not.trit(1), Trit::Z);
        assert_eq!(not.trit(2), Trit::N);
    }

    #[test]
    fn tryte_tshl() {
        let t = Tryte::new([
            Trit::P,
            Trit::N,
            Trit::P,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ]);
        let shifted = t.tshl(1);
        assert_eq!(shifted.trit(0), Trit::Z); // filled with zero
        assert_eq!(shifted.trit(1), Trit::P); // original[0]
        assert_eq!(shifted.trit(2), Trit::N); // original[1]
    }

    #[test]
    fn tryte_tshr_positive() {
        // sign() scans from end, finds P at index 8, returns P
        // tshr(1): shifted[i] = original[i+1] if i+1 < 9, else use sign (P)
        // Result: [Z, Z, Z, Z, Z, Z, Z, P, P]
        let t = Tryte::new([
            Trit::N,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::P,
        ]);
        let shifted = t.tshr(1);
        assert_eq!(shifted.trit(0), Trit::Z); // original[1] = Z
        assert_eq!(shifted.trit(7), Trit::P); // original[8] = P
        assert_eq!(shifted.trit(8), Trit::P); // filled with sign (P)
    }

    #[test]
    fn tryte_tshr_negative() {
        let t = Tryte::new([
            Trit::P,
            Trit::N,
            Trit::P,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::N,
        ]);
        let shifted = t.tshr(1);
        assert_eq!(shifted.trit(8), Trit::N); // filled with sign (N)
    }

    #[test]
    fn tryte_tlshr() {
        let t = Tryte::new([
            Trit::P,
            Trit::N,
            Trit::P,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ]);
        let shifted = t.tlshr(1);
        assert_eq!(shifted.trit(0), Trit::N); // original[1]
        assert_eq!(shifted.trit(1), Trit::P); // original[2]
        assert_eq!(shifted.trit(8), Trit::Z); // always filled with zero
    }

    #[test]
    fn tryte_tconsensus() {
        let t1 = Tryte::new([
            Trit::N,
            Trit::P,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ]);
        let t2 = Tryte::new([
            Trit::N,
            Trit::P,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ]);
        let cons = t1.tconsensus(t2);
        assert_eq!(cons.trit(0), Trit::N); // consensus(N, N) = N (agree)
        assert_eq!(cons.trit(1), Trit::P); // consensus(P, P) = P (agree)
        assert_eq!(cons.trit(2), Trit::Z); // consensus(Z, Z) = Z
    }

    #[test]
    fn tryte_tconsensus_disagree() {
        let t1 = Tryte::new([
            Trit::N,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ]);
        let t2 = Tryte::new([
            Trit::P,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ]);
        let cons = t1.tconsensus(t2);
        assert_eq!(cons.trit(0), Trit::N); // consensus(N, P) = N (disagree → N)
    }

    #[test]
    fn tryte_display() {
        let t = Tryte::new([
            Trit::N,
            Trit::Z,
            Trit::P,
            Trit::N,
            Trit::Z,
            Trit::P,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ]);
        assert_eq!(t.to_string(), "NZPNZPZZZ");
    }

    #[test]
    fn word27_new() {
        let trits = [Trit::P; 27];
        let w = Word27::new(trits);
        for i in 0..27 {
            assert_eq!(w.trit(i), Trit::P);
        }
    }

    #[test]
    fn word27_zero() {
        let w = Word27::zero();
        for i in 0..27 {
            assert_eq!(w.trit(i), Trit::Z);
        }
    }

    #[test]
    fn word27_sign() {
        let mut trits = [Trit::Z; 27];
        trits[26] = Trit::N;
        let w = Word27::new(trits);
        assert_eq!(w.sign(), Trit::N);
    }

    #[test]
    fn word27_negate() {
        let mut trits = [Trit::Z; 27];
        trits[0] = Trit::P;
        let w = Word27::new(trits);
        let neg = w.negate();
        assert_eq!(neg.trit(0), Trit::N);
        for i in 1..27 {
            assert_eq!(neg.trit(i), Trit::Z);
        }
    }

    #[test]
    fn word27_display() {
        let mut trits = [Trit::Z; 27];
        trits[0] = Trit::N;
        trits[1] = Trit::P;
        let w = Word27::new(trits);
        let display = w.to_string();
        assert_eq!(display.len(), 27);
        assert_eq!(display.chars().next(), Some('N'));
        assert_eq!(display.chars().nth(1), Some('P'));
    }

    // <- Compile-time checks

    const _: Tryte = Tryte::zero();
    const _: Tryte = Tryte::from_trits([Trit::Z; 9]);

    const _: Word27 = Word27::zero();
    const _: Word27 = Word27::from_trits([Trit::Z; 27]);
}
