use super::{CheckedDiv, Tryte, Word, Word27, Word54};
use crate::ops::ripple_add;
use crate::trit::Trit;
use core::ops;

impl<const N: usize> ops::Add for Word<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(ripple_add(self.0, rhs.0))
    }
}

impl<const N: usize> ops::Sub for Word<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl<const N: usize> ops::Mul for Word<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut product = Self::zero();

        for (shift, trit) in rhs.0.iter().copied().enumerate() {
            let partial = self.tshl(shift);

            product = match trit {
                Trit::N => product - partial,
                Trit::Z => product,
                Trit::P => product + partial,
            };
        }

        product
    }
}

impl CheckedDiv for Tryte {
    type Output = Self;

    fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs == Self::zero() {
            return None;
        }

        Self::from_int(self.to_int() / rhs.to_int()).ok()
    }
}

impl CheckedDiv for Word27 {
    type Output = Self;

    fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs == Self::zero() {
            return None;
        }

        Self::from_int(self.to_int() / rhs.to_int()).ok()
    }
}

impl CheckedDiv for Word54 {
    type Output = Self;

    fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs == Self::zero() {
            return None;
        }

        Self::from_int(self.to_int() / rhs.to_int()).ok()
    }
}

impl ops::Div for Tryte {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(rhs)
            .unwrap_or_else(|| panic!("Tryte: division by zero"))
    }
}

impl ops::Div for Word27 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(rhs)
            .unwrap_or_else(|| panic!("Word27: division by zero"))
    }
}

impl ops::Div for Word54 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(rhs)
            .unwrap_or_else(|| panic!("Word54: division by zero"))
    }
}
