use crate::{classify::*, const_interval, interval::*};
use libm;

fn rem_euclid_2(x: f64) -> f64 {
    if 2.0 * (x / 2.0).floor() == x {
        0.0
    } else {
        1.0
    }
}
macro_rules! impl_log {
    ($(#[$meta:meta])* $f:ident, $f_real:expr) => {
        $(#[$meta])*
        #[allow(dead_code)]
        #[must_use]
        pub fn $f(self) -> Self {
            // See the comment in atanh_impl.
            const DOM: Interval = const_interval!(0.0, f64::INFINITY);
            let x = self.intersection(DOM);

            let (a, b) = (x.inf, x.sup);
            if x.is_empty() || b <= 0.0 {
                return Self::EMPTY;
            }

            Self::with_infsup_raw($f_real(a), $f_real(b))
        }
    };
}

macro_rules! impl_mono_inc {
    ($(#[$meta:meta])* $f:ident, $f_real:expr) => {
        $(#[$meta])*
        #[must_use]
        pub fn $f(self) -> Self {
            if self.is_empty() {
                return self;
            }

            Self::with_infsup_raw($f_real(self.inf), $f_real(self.sup))
        }
    };
}

impl Interval {
    /// Returns the inverse cosine of `self`.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain      | Range      |
    /// | ----------- | ---------- |
    /// | $\[-1, 1\]$ | $\[0, π\]$ |
    #[must_use]
    pub fn acos(self) -> Self {
        const DOM: Interval = const_interval!(-1.0, 1.0);
        let x = self.intersection(DOM);

        if x.is_empty() {
            return x;
        }

        Self::with_infsup_raw(f64::acos(x.sup), f64::acos(x.inf))
    }

    /// Returns the inverse hyperbolic cosine of `self`.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain    | Range     |
    /// | --------- | --------- |
    /// | $\[1, ∞)$ | $\[0, ∞)$ |
    #[must_use]
    pub fn acosh(self) -> Self {
        const DOM: Interval = const_interval!(1.0, f64::INFINITY);
        let x = self.intersection(DOM);

        if x.is_empty() {
            return x;
        }

        Self::with_infsup_raw(f64::acosh(x.inf), f64::acosh(x.sup))
    }


    /// Returns the inverse sine of `self`.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain      | Range           |
    /// | ----------- | --------------- |
    /// | $\[-1, 1\]$ | $\[-π/2, π/2\]$ |
    #[must_use]
    pub fn asin(self) -> Self {
        const DOM: Interval = const_interval!(-1.0, 1.0);
        let x = self.intersection(DOM);

        if x.is_empty() {
            return x;
        }

        Self::with_infsup_raw(f64::asin(x.inf), f64::asin(x.sup))
    }

    impl_mono_inc!(
        /// Returns the inverse hyperbolic sine of `self`.
        ///
        /// The domain and the range of the point function are:
        ///
        /// | Domain | Range |
        /// | ------ | ----- |
        /// | $\R$   | $\R$  |
        asinh,
        f64::asinh
    );
    impl_mono_inc!(
        /// Returns the inverse tangent of `self`.
        ///
        /// The domain and the range of the point function are:
        ///
        /// | Domain | Range         |
        /// | ------ | ------------- |
        /// | $\R$   | $(-π/2, π/2)$ |
        atan,
		f64::atan
    );

    /// Returns the angle of the point $(\rhs, \self)$ measured counterclockwise from the positive
    /// $x$-axis in the Euclidean plane.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain                | Range      |
    /// | --------------------- | ---------- |
    /// | $\R^2 ∖ \set{(0, 0)}$ | $(-π, π\]$ |
    #[must_use]
    #[allow(clippy::many_single_char_names)]
    pub fn atan2(self, rhs: Self) -> Self {
        let (x, y) = (rhs, self);
        let a = x.inf;
        let b = x.sup;
        let c = y.inf;
        let d = y.sup;

        use IntervalClass2::*;
        match x.classify2(y) {
            E_E | E_M | E_N0 | E_N1 | E_P0 | E_P1 | E_Z | M_E | N0_E | N1_E | P0_E | P1_E | Z_E
            | Z_Z => Self::EMPTY,
            M_M | M_N0 | N0_M | N0_N0 => Self::with_infsup_raw(-Self::PI.sup, Self::PI.sup),

            // First quadrant
            P0_P0 => Self::with_infsup_raw(0.0, Self::FRAC_PI_2.sup),
            P0_P1 | P1_P0 | P1_P1 | P1_Z | Z_P1 => Self::with_infsup_raw(f64::atan2(c, b), f64::atan2(d, a)),

            // First & second quadrant
            M_P0 | M_Z => Self::with_infsup_raw(0.0, Self::PI.sup),
            M_P1 => Self::with_infsup_raw(f64::atan2(c, b), f64::atan2(c, a)),

            // Second quadrant
            N0_P0=> Self::with_infsup_raw(Self::FRAC_PI_2.inf, Self::PI.sup),
            N0_P1 | N1_P1 => Self::with_infsup_raw(f64::atan2(d, b), f64::atan2(c, a)),
            N1_P0 => Self::with_infsup_raw(f64::atan2(d, b), Self::PI.sup),

            // Second & third quadrant
            //N0_M => See above.
            N1_M | N1_N0 => Self::with_infsup_raw(-Self::PI.sup, Self::PI.sup),

            // Third quadrant
            //N0_N0 => See above.
            N0_N1 | N1_N1 => Self::with_infsup_raw(f64::atan2(d, a), f64::atan2(c, b)),
            //N1_N0 => See above.

            // Third & fourth quadrant
            //M_N0 => See above.
            M_N1 => Self::with_infsup_raw(f64::atan2(d, a), f64::atan2(d, b)),

            // Fourth quadrant
            P0_N0 => Self::with_infsup_raw(-Self::FRAC_PI_2.sup, 0.0),
            P0_N1 | P1_N0 | P1_N1 | Z_N1 => Self::with_infsup_raw(f64::atan2(c, a), f64::atan2(d, b)),

            // Fourth & first quadrant
            P0_M | Z_M => Self::with_infsup_raw(-Self::FRAC_PI_2.sup, Self::FRAC_PI_2.sup),
            P1_M => Self::with_infsup_raw(f64::atan2(c, a), f64::atan2(d, a)),

            // X axis
            //M_Z => See above.
            N0_Z => Self::PI,
            // The next case cannot be merged with N1_P0 unless we replace -0.0 with +0.0
            // since IEEE 754/MPFR's atan2 returns ±π for y = ±0.0, x < 0.0, while we want only +π.
            N1_Z => Self::PI,
            P0_Z => Self::zero(),
            //P1_Z => See above.

            // Y axis
            //Z_M => See above.
            Z_N0 => -Self::FRAC_PI_2,
            //Z_N1 => See above.
            Z_P0 => Self::FRAC_PI_2,
            //Z_P1 => See above.
        }
    }

    /// Returns the inverse hyperbolic tangent of `self`.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain    | Range |
    /// | --------- | ----- |
    /// | $(-1, 1)$ | $\R$  |
    #[must_use]
    pub fn atanh(self) -> Self {
        // Mathematically, the domain of atanh is (-1.0, 1.0), not [-1.0, 1.0].
        // However, IEEE 754/MPFR's atanh returns ±infinity for ±1.0,
        // (and signals the divide-by-zero exception), so we will make use of that.
        const DOM: Interval = const_interval!(-1.0, 1.0);
        let x = self.intersection(DOM);

        let a = x.inf;
        let b = x.sup;
        if x.is_empty() || b <= -1.0 || a >= 1.0 {
            return Self::EMPTY;
        }

        Self::with_infsup_raw(f64::atanh(a), f64::atanh(b))
    }

    /// Returns the cosine of `self`.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain | Range       |
    /// | ------ | ----------- |
    /// | $\R$   | $\[-1, 1\]$ |
    #[must_use]
    pub fn cos(self) -> Self {
        if self.is_empty() {
            return self;
        }

        let a = self.inf;
        let b = self.sup;
        let q_nowrap = (self / Self::PI).floor();
        let qa = q_nowrap.inf;
        let qb = q_nowrap.sup;
        // n and q are valid for small values.
        let n = if a == b {
            // For strict test cases on huge values.
            0.0
        } else {
            qb - qa
        };
        let q = rem_euclid_2(qa);

        // Overestimation is fine.
        if n == 0.0 {
            if q == 0.0 {
                // monotonically decreasing
                Self::with_infsup_raw(f64::cos(b), f64::cos(a))
            } else {
                // monotonically increasing
                Self::with_infsup_raw(f64::cos(a), f64::cos(b))
            }
        } else if n <= 1.0 {
            if q == 0.0 {
                // decreasing, then increasing
                Self::with_infsup_raw(-1.0, f64::cos(a).max(f64::cos(b)))
            } else {
                // increasing, then decreasing
                Self::with_infsup_raw(f64::cos(a).min(f64::cos(b)), 1.0)
            }
        } else {
            const_interval!(-1.0, 1.0)
        }
    }

    /// Returns the hyperbolic cosine of `self`.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain | Range     |
    /// | ------ | --------- |
    /// | $\R$   | $\[1, ∞)$ |
    #[must_use]
    pub fn cosh(self) -> Self {
        if self.is_empty() {
            return self;
        }

        let a = self.inf;
        let b = self.sup;
        if b < 0.0 {
            Self::with_infsup_raw(f64::cosh(b), f64::cosh(a))
        } else if a > 0.0 {
            Self::with_infsup_raw(f64::cosh(a), f64::cosh(b))
        } else {
            Self::with_infsup_raw(1.0, f64::cosh((-a).max(b)))
        }
    }

    impl_mono_inc!(
        /// Returns `self` raised to the power of $\e$.
        ///
        /// The domain and the range of the point function are:
        ///
        /// | Domain | Range    |
        /// | ------ | -------- |
        /// | $\R$   | $(0, ∞)$ |
        exp,
        f64::exp
    );
	impl_mono_inc!(
        /// Returns `self` raised to the power of $\e$.
        ///
        /// The domain and the range of the point function are:
        ///
        /// | Domain | Range    |
        /// | ------ | -------- |
        /// | $\R$   | $(0, ∞)$ |
        exp10,
        libm::exp10
    );
    impl_mono_inc!(
        /// Returns `self` raised to the power of 2.
        ///
        /// The domain and the range of the point function are:
        ///
        /// | Domain | Range    |
        /// | ------ | -------- |
        /// | $\R$   | $(0, ∞)$ |
        exp2,
        f64::exp2
    );

    impl_log!(
        /// Returns the natural logarithm of `self`.
        ///
        /// The domain and the range of the point function are:
        ///
        /// | Domain   | Range |
        /// | -------- | ----- |
        /// | $(0, ∞)$ | $\R$  |
        ln,
        f64::ln
    );
    impl_log!(
        /// Returns the base-10 logarithm of `self`.
        ///
        /// The domain and the range of the point function are:
        ///
        /// | Domain   | Range |
        /// | -------- | ----- |
        /// | $(0, ∞)$ | $\R$  |
        log10,
        libm::log10
    );
    impl_log!(
        /// Returns the base-2 logarithm of `self`.
        ///
        /// The domain and the range of the point function are:
        ///
        /// | Domain   | Range |
        /// | -------- | ----- |
        /// | $(0, ∞)$ | $\R$  |
        log2,
        f64::log2
    );

    /// Returns `self` raised to the power of `rhs`.
    ///
    /// The point function is defined as follows:
    ///
    /// $$
    /// x^y = \begin{cases}
    ///   0           & \for x = 0 ∧ y > 0, \\\\
    ///   e^{y \ln x} & \for x > 0.
    ///  \end{cases}
    /// $$
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain                              | Range     |
    /// | ----------------------------------- | --------- |
    /// | $((0, ∞) × \R) ∪ (\set 0 × (0, ∞))$ | $\[0, ∞)$ |
    #[must_use]
    pub fn pow(self, rhs: Self) -> Self {
		const DOM: Interval = const_interval!(0.0, f64::INFINITY);
        let x = self.intersection(DOM);

        if x.either_empty(rhs) {
            return Self::EMPTY;
        }

        let a = x.inf;
        let b = x.sup;
        let c = rhs.inf;
        let d = rhs.sup;

        if d <= 0.0 {
            if b == 0.0 {
                return Self::EMPTY;
            }

            if b < 1.0 {
                Self::with_infsup_raw(f64::powf(b, d), f64::powf(a, c))
            } else if a > 1.0 {
                Self::with_infsup_raw(f64::powf(b, c), f64::powf(a, d))
            } else {
                Self::with_infsup_raw(f64::powf(b, c), f64::powf(a, c))
            }
        } else if c > 0.0 {
            if b < 1.0 {
                Self::with_infsup_raw(f64::powf(a, d), f64::powf(b, c))
            } else if a > 1.0 {
                Self::with_infsup_raw(f64::powf(a, c), f64::powf(b, d))
            } else {
                Self::with_infsup_raw(f64::powf(a, d), f64::powf(b, d))
            }
        } else {
            if b == 0.0 {
                return Self::zero();
            }

            let z_ac = f64::powf(a, c);
            let z_ad = f64::powf(a, d);
            let z_bc = f64::powf(b, c);
            let z_bd = f64::powf(b, d);

            Self::with_infsup_raw(z_ad.min(z_bc), z_ac.max(z_bd))
        }
    }

    /// Returns `self` raised to the power of `rhs`.
    ///
    /// The point functions are indexed by $n$, and are defined as follows:
    ///
    /// $$
    /// x^n = \begin{cases}
    ///   \overbrace{x × ⋯ × x}^{n \text{ copies}} & \for n > 0, \\\\
    ///   1          & \for n = 0, \\\\
    ///   1 / x^{-n} & \for n < 0.
    ///  \end{cases}
    /// $$
    ///
    /// The domains and the ranges of the point functions are:
    ///
    /// |                | Domain        | Range         |
    /// | -------------- | ------------- | ------------- |
    /// | $n > 0$, odd   | $\R$          | $\R$          |
    /// | $n > 0$, even  | $\R$          | $\[0, ∞)$     |
    /// | $n = 0$        | $\R$          | $\set 1$      |
    /// | $n < 0$, odd   | $\R ∖ \set 0$ | $\R ∖ \set 0$ |
    /// | $n < 0$, even  | $\R ∖ \set 0$ | $(0, ∞)$      |
    #[must_use]
    pub fn powi(self, rhs: i32) -> Self {
        if self.is_empty() {
            return self;
        }

        let mut a = self.inf;
        let mut b = self.sup;

        #[allow(clippy::collapsible_else_if, clippy::collapsible_if)]
        if rhs < 0 {
			if a == 0.0 && b == 0.0 {
                return Self::EMPTY;
            }

            if rhs % 2 == 0 {
                let abs = self.abs();
				Self::with_infsup_raw(f64::powi(abs.sup, rhs), f64::powi(abs.inf, rhs))
            } else {
                if a < 0.0 && b > 0.0 {
                    Self::ENTIRE
                } else {
                    if a == 0.0 {
                        a = 0.0; // [0, b]
                    }
                    if b == 0.0 {
                        b = -0.0; // [a, 0]
                    }
                    Self::with_infsup_raw(f64::powi(b, rhs), f64::powi(a, rhs))
                }
            }
        } else {
            if rhs % 2 == 0 {
                let abs = self.abs();
				Self::with_infsup_raw(f64::powi(abs.inf, rhs), f64::powi(abs.sup, rhs))
            } else {
				Self::with_infsup_raw(f64::powi(a, rhs), f64::powi(b, rhs))
            }
        }
    }

    /// Returns the sine of `self`.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain | Range       |
    /// | ------ | ----------- |
    /// | $\R$   | $\[-1, 1\]$ |
    #[must_use]
    pub fn sin(self) -> Self {
        if self.is_empty() {
            return self;
        }

        let a = self.inf;
        let b = self.sup;
        let q_nowrap = (self / Self::FRAC_PI_2).floor();
        let qa = q_nowrap.inf;
        let qb = q_nowrap.sup;
        let n = if a == b { 0.0 } else { qb - qa };
        let q = qa.rem_euclid(4.0);

        if q == 0.0 && n < 1.0 || q == 3.0 && n < 2.0 {
            // monotonically increasing
            Self::with_infsup_raw(f64::sin(a), f64::sin(b))
        } else if q == 1.0 && n < 2.0 || q == 2.0 && n < 1.0 {
            // monotonically decreasing
            Self::with_infsup_raw(f64::sin(b), f64::sin(a))
        } else if q == 0.0 && n < 3.0 || q == 3.0 && n < 4.0 {
            // increasing, then decreasing
            Self::with_infsup_raw(f64::sin(a).min(f64::sin(b)), 1.0)
        } else if q == 1.0 && n < 4.0 || q == 2.0 && n < 3.0 {
            // decreasing, then increasing
            Self::with_infsup_raw(-1.0, f64::sin(a).max(f64::sin(b)))
        } else {
            const_interval!(-1.0, 1.0)
        }
    }

    impl_mono_inc!(
        /// Returns the hyperbolic sine of `self`.
        ///
        /// The domain and the range of the point function are:
        ///
        /// | Domain | Range |
        /// | ------ | ----- |
        /// | $\R$   | $\R$  |
        sinh,
        f64::sinh
    );

    /// Returns the tangent of `self`.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain                            | Range |
    /// | --------------------------------- | ----- |
    /// | $\R ∖ \set{(n + 1/2) π ∣ n ∈ \Z}$ | $\R$  |
    #[must_use]
    pub fn tan(self) -> Self {
        if self.is_empty() {
            return self;
        }

        let a = self.inf;
        let b = self.sup;
        let q_nowrap = (self / Self::FRAC_PI_2).floor();
        let qa = q_nowrap.inf;
        let qb = q_nowrap.sup;
        let n = if a == b { 0.0 } else { qb - qa };
        let q = rem_euclid_2(qa);

        let cont =
            qb != f64::INFINITY && b <= (Self::with_infsup_raw(qb, qb) * Self::FRAC_PI_2).inf;
        if q == 0.0 && (n < 1.0 || n == 1.0 && cont) || q == 1.0 && (n < 2.0 || n == 2.0 && cont) {
            // In case of overflow, the decoration must be corrected by the caller.
            Self::with_infsup_raw(f64::tan(a), f64::tan(b))
        } else {
            Self::ENTIRE
        }
    }

    impl_mono_inc!(
        /// Returns the hyperbolic tangent of `self`.
        ///
        /// The domain and the range of the point function are:
        ///
        /// | Domain | Range     |
        /// | ------ | --------- |
        /// | $\R$   | $(-1, 1)$ |
        tanh,
        f64::tanh
    );
}


#[cfg(test)]
mod tests {
    use crate::*;
    use Interval as I;

    #[test]
    fn tan() {
        // a, b ∈ (-π/2, π/2)
        assert!(interval!(std::f64::consts::FRAC_PI_4, I::FRAC_PI_2.inf)
            .unwrap()
            .tan()
            .is_common_interval());
        assert!(interval!(-std::f64::consts::FRAC_PI_4, I::FRAC_PI_2.inf)
            .unwrap()
            .tan()
            .is_common_interval());

        // a, b ∈ (-3π/2, -π/2)
        assert!(
            interval!(-3.0 * std::f64::consts::FRAC_PI_4, -I::FRAC_PI_2.sup)
                .unwrap()
                .tan()
                .is_common_interval()
        );
        assert!(
            interval!(-5.0 * std::f64::consts::FRAC_PI_4, -I::FRAC_PI_2.sup)
                .unwrap()
                .tan()
                .is_common_interval()
        );
    }
}
