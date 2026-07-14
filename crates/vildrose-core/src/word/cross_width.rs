use super::{CheckedDiv, Tryte, Word27, Word54};
use crate::ops::widen;
use core::ops;

macro_rules! impl_widening_arithmetic {
    ($narrow:ty => $wide:ty, $wide_width:literal) => {
        impl From<$narrow> for $wide {
            fn from(value: $narrow) -> Self {
                <$wide>::new(widen::<{ <$narrow>::TRIT_COUNT }, $wide_width>(
                    value.into_trits(),
                ))
            }
        }

        impl ops::Add<$wide> for $narrow {
            type Output = $wide;

            fn add(self, rhs: $wide) -> Self::Output {
                let lhs: $wide = self.into();
                lhs + rhs
            }
        }

        impl ops::Add<$narrow> for $wide {
            type Output = $wide;

            fn add(self, rhs: $narrow) -> Self::Output {
                let rhs: $wide = rhs.into();
                self + rhs
            }
        }

        impl ops::Sub<$wide> for $narrow {
            type Output = $wide;

            fn sub(self, rhs: $wide) -> Self::Output {
                let lhs: $wide = self.into();
                lhs - rhs
            }
        }

        impl ops::Sub<$narrow> for $wide {
            type Output = $wide;

            fn sub(self, rhs: $narrow) -> Self::Output {
                let rhs: $wide = rhs.into();
                self - rhs
            }
        }

        impl ops::Mul<$wide> for $narrow {
            type Output = $wide;

            fn mul(self, rhs: $wide) -> Self::Output {
                let lhs: $wide = self.into();
                lhs * rhs
            }
        }

        impl ops::Mul<$narrow> for $wide {
            type Output = $wide;

            fn mul(self, rhs: $narrow) -> Self::Output {
                let rhs: $wide = rhs.into();
                self * rhs
            }
        }

        impl CheckedDiv<$wide> for $narrow {
            type Output = $wide;

            fn checked_div(self, rhs: $wide) -> Option<Self::Output> {
                let lhs: $wide = self.into();
                <$wide as CheckedDiv<$wide>>::checked_div(lhs, rhs)
            }
        }

        impl CheckedDiv<$narrow> for $wide {
            type Output = $wide;

            fn checked_div(self, rhs: $narrow) -> Option<Self::Output> {
                let rhs: $wide = rhs.into();
                <$wide as CheckedDiv<$wide>>::checked_div(self, rhs)
            }
        }

        impl ops::Div<$wide> for $narrow {
            type Output = $wide;

            fn div(self, rhs: $wide) -> Self::Output {
                let lhs: $wide = self.into();
                lhs / rhs
            }
        }

        impl ops::Div<$narrow> for $wide {
            type Output = $wide;

            fn div(self, rhs: $narrow) -> Self::Output {
                let rhs: $wide = rhs.into();
                self / rhs
            }
        }
    };
}

impl_widening_arithmetic!(Tryte => Word27, 27);
impl_widening_arithmetic!(Tryte => Word54, 54);
impl_widening_arithmetic!(Word27 => Word54, 54);
