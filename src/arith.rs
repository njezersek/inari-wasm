use crate::{interval::*, classify::*};

use forward_ref::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self {
        // [-b, -a] = [b; -a]
        Self {
            inf: -self.sup,
			sup: -self.inf,
        }
    }
}

forward_ref_unop!(impl Neg, neg for Interval);


impl Add for Interval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        // [a + c, b + d] = [-a - c; b + d] = [-a; b] .+ [-c; d]
        Self { 
			inf: self.inf + rhs.inf,
			sup: self.sup + rhs.sup,
		}
    }
}

forward_ref_binop!(impl Add, add for Interval, Interval);


impl Sub for Interval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        // [a - d, b - c]
        Self { 
			inf: self.inf - rhs.sup,
			sup: self.sup - rhs.inf,
		}
    }
}

forward_ref_binop!(impl Sub, sub for Interval, Interval);


impl Mul for Interval {
    type Output = Self;

    #[allow(clippy::many_single_char_names)]
    fn mul(self, rhs: Self) -> Self {
        // [a, b] * [c, d] =
        //
        //    |      M     |      N     |      P     |  Z
        // ---+------------+------------+------------+-----
        //  M |     *1     | [b*c, a*c] | [a*d, b*d] | {0}
        //  N | [a*d, a*c] | [b*d, a*c] | [a*d, b*c] | {0}
        //  P | [b*c, b*d] | [b*c, a*d] | [a*c, b*d] | {0}
        //  Z |     {0}    |     {0}    |     {0}    | {0}
        // *1 [min{a*d, b*c}, max{a*c, b*d}]

		let (a, b) = (self.inf, self.sup);
		let (c, d) = (rhs.inf, rhs.sup);

        use IntervalClass2::*;
        match self.classify2(rhs) {
            E_E | E_M | E_N0 | E_N1 | E_P0 | E_P1 | E_Z | M_E | N0_E | N1_E | P0_E | P1_E | Z_E => {
                Self::EMPTY
            }
            M_Z | N0_Z | N1_Z | P0_Z | P1_Z | Z_M | Z_N0 | Z_N1 | Z_P0 | Z_P1 | Z_Z => Self::zero(),
            M_M => {
                // M * M => [min(a*d, b*c), max(a*c, b*d)]
                Self { inf: f64::min(a * d, b * c), sup: f64::max(a * c, b * d) }
            }
            M_N0 | M_N1 => {
                // M * N => [b*c, a*c]
                Self { inf: b * c, sup: a * c }
            }
            M_P0 | M_P1 => {
                // M * P => [a*d, b*d]
                Self { inf: a * d, sup: b * d }
            }
            N0_M | N1_M => {
                // N * M => [a*d, a*c]
                Self { inf: a * d, sup: a * c }
            }
            N0_N0 | N0_N1 | N1_N0 | N1_N1 => {
                // N * N => [b*d, a*c]
                Self { inf: b * d, sup: a * c }
            }
            N0_P0 | N0_P1 | N1_P0 | N1_P1 => {
                // N * P => [a*d, b*c]
                Self { inf: a * d, sup: b * c }
            }
            P0_M | P1_M => {
                // P * M => [b*c, b*d]
                Self { inf: b * c, sup: b * d }
            }
            P0_N0 | P0_N1 | P1_N0 | P1_N1 => {
                // P * N => [b*c, a*d]
                Self { inf: b * c, sup: a * d }
            }
            P0_P0 | P0_P1 | P1_P0 | P1_P1 => {
                // P * P => [a*c, b*d]
                Self { inf: a * c, sup: b * d }
            }
        }
    }
}

forward_ref_binop!(impl Mul, mul for Interval, Interval);


impl Div for Interval {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        // [a, b] / [c, d] =
        //
        //    |  M  |     N0    |     N1     |     P0    |     P1     | Z
        // ---+-----+-----------+------------+-----------+------------+---
        //  M |  ℝ  |     ℝ     | [b/d, a/d] |     ℝ     | [a/c, b/c] | ∅
        //  N |  ℝ  | [b/c, +∞] | [b/c, a/d] | [-∞, b/d] | [a/c, b/d] | ∅
        //  P |  ℝ  | [-∞, a/c] | [b/d, a/c] | [a/d, +∞] | [a/d, b/c] | ∅
        //  Z | {0} |    {0}    |     {0}    |    {0}    |     {0}    | ∅


		let (a, b) = (self.inf, self.sup);
		let (c, d) = (rhs.inf, rhs.sup);

        use IntervalClass2::*;
        match self.classify2(rhs) {
            E_E | E_M | E_N0 | E_N1 | E_P0 | E_P1 | E_Z | M_E | M_Z | N0_E | N0_Z | N1_E | N1_Z
            | P0_E | P0_Z | P1_E | P1_Z | Z_E | Z_Z => Self::EMPTY,
            M_M | M_N0 | M_P0 | N0_M | N1_M | P0_M | P1_M => Self::ENTIRE,
            Z_M | Z_N0 | Z_N1 | Z_P0 | Z_P1 => Self::zero(),
            M_N1 => {
                // M / N1 => [b/d, a/d]
                Self { inf: b / d, sup: a / d }
            }
            M_P1 => {
                // M / P1 => [a/c, b/c]
                Self { inf: a / c, sup: b / c }
            }
            N0_N0 | N1_N0 => {
                // N / N0 => [b/c, +∞]
				Self { inf: b / c, sup: f64::INFINITY }
            }
            N0_N1 | N1_N1 => {
                // N / N1 => [b/c, a/d]
				Self { inf: b / c, sup: a / d }
            }
            N0_P0 | N1_P0 => {
                // N / P0 => [-∞, b/d] 
				Self { inf: f64::NEG_INFINITY, sup: b / d }
            }
            N0_P1 | N1_P1 => {
                // N / P1 => [a/c, b/d]
				Self { inf: a / c, sup: b / d }
            }
            P0_N0 | P1_N0 => {
                // P / N0 => [-∞, a/c]
				Self { inf: f64::NEG_INFINITY, sup: a / c }
            }
            P0_N1 | P1_N1 => {
                // P / N1 => [b/d, a/c] 
				Self { inf: b / d, sup: a / c }
            }
            P0_P0 | P1_P0 => {
                // P / P0 => [a/d, +∞]
				Self { inf: a / d, sup: f64::INFINITY }
            }
            P0_P1 | P1_P1 => {
                // P / P1 => [a/d, b/c]
				Self { inf: a / d, sup: b / c }
            }
        }
    }
}

forward_ref_binop!(impl Div, div for Interval, Interval);

macro_rules! impl_op_assign {
    ($OpAssign:ident, $op_assign:ident, $op:ident) => {
        impl $OpAssign for Interval {
            fn $op_assign(&mut self, rhs: Self) {
                *self = self.$op(rhs);
            }
        }

        forward_ref_op_assign!(impl $OpAssign, $op_assign for Interval, Interval);
    };
}

impl_op_assign!(AddAssign, add_assign, add);
impl_op_assign!(SubAssign, sub_assign, sub);
impl_op_assign!(MulAssign, mul_assign, mul);
impl_op_assign!(DivAssign, div_assign, div);

#[cfg(test)]
mod tests {
    use crate::*;
    use Interval as I;

    #[test]
    fn add_assign() {
        let mut i = const_interval!(3.0, 4.0);
        i += const_interval!(1.0, 2.0);
        assert_eq!(i, const_interval!(4.0, 6.0));
    }

    #[test]
    fn sub_assign() {
        let mut i = const_interval!(3.0, 4.0);
        i -= const_interval!(1.0, 2.0);
        assert_eq!(i, const_interval!(1.0, 3.0));
    }

    #[test]
    fn mul_assign() {
        let mut i = const_interval!(3.0, 4.0);
        i *= const_interval!(1.0, 2.0);
        assert_eq!(i, const_interval!(3.0, 8.0));
    }

    #[test]
    fn div_assign() {
        let mut i = const_interval!(3.0, 4.0);
        i /= const_interval!(1.0, 2.0);
        assert_eq!(i, const_interval!(1.5, 4.0));
    }

    #[test]
    fn empty() {
        assert!((-I::EMPTY).is_empty());

        assert!((I::EMPTY + I::PI).is_empty());
        assert!((I::PI + I::EMPTY).is_empty());

        assert!((I::EMPTY - I::PI).is_empty());
        assert!((I::PI - I::EMPTY).is_empty());

        assert!((I::EMPTY * I::PI).is_empty());
        assert!((I::PI * I::EMPTY).is_empty());

        assert!((I::EMPTY / I::PI).is_empty());
        assert!((I::PI / I::EMPTY).is_empty());
    }

    #[allow(clippy::op_ref)]
    #[test]
    fn ref_type_args() {
        const E: I = I::EMPTY;

        let _ = -&E;

        let _ = &E + E;
        let _ = E + &E;
        let _ = &E + &E;

        let _ = &E - E;
        let _ = E - &E;
        let _ = &E - &E;

        let _ = &E * E;
        let _ = E * &E;
        let _ = &E * &E;

        let _ = &E / E;
        let _ = E / &E;
        let _ = &E / &E;


        let mut e = I::EMPTY;
        e += &E;
        e -= &E;
        e *= &E;
        e /= &E;
    }
}