#![allow(clippy::approx_constant)]

use crate::{const_interval, interval::*};

impl Interval {
    /// $∅$, the empty set.
    pub const EMPTY: Self = Interval{inf: f64::NAN, sup: f64::NAN};

    /// $\[-∞, +∞\]$.
    pub const ENTIRE: Self = const_interval!(f64::NEG_INFINITY, f64::INFINITY);

    /// The tightest interval enclosing $\e$, the base of natural logarithms.
    pub const E: Self = const_interval!(2.718281828459045, 2.7182818284590455);

    /// The tightest interval enclosing $1 / π$.
    pub const FRAC_1_PI: Self = const_interval!(0.31830988618379064, 0.3183098861837907);

    /// The tightest interval enclosing $1 / \sqrt{2}$.
    pub const FRAC_1_SQRT_2: Self = const_interval!(0.7071067811865475, 0.7071067811865476);

    /// The tightest interval enclosing $2 / π$.
    pub const FRAC_2_PI: Self = const_interval!(0.6366197723675813, 0.6366197723675814);

    /// The tightest interval enclosing $2 / \sqrt{π}$.
    pub const FRAC_2_SQRT_PI: Self = const_interval!(1.1283791670955126, 1.1283791670955128);

    /// The tightest interval enclosing $π / 2$.
    pub const FRAC_PI_2: Self = const_interval!(1.5707963267948966, 1.5707963267948968);

    /// The tightest interval enclosing $π / 3$.
    pub const FRAC_PI_3: Self = const_interval!(1.0471975511965976, 1.0471975511965979);

    /// The tightest interval enclosing $π / 4$.
    pub const FRAC_PI_4: Self = const_interval!(0.7853981633974483, 0.7853981633974484);

    /// The tightest interval enclosing $π / 6$.
    pub const FRAC_PI_6: Self = const_interval!(0.5235987755982988, 0.5235987755982989);

    /// The tightest interval enclosing $π / 8$.
    pub const FRAC_PI_8: Self = const_interval!(0.39269908169872414, 0.3926990816987242);

    /// The tightest interval enclosing $\ln 10$.
    pub const LN_10: Self = const_interval!(2.3025850929940455, 2.302585092994046);

    /// The tightest interval enclosing $\ln 2$.
    pub const LN_2: Self = const_interval!(0.6931471805599453, 0.6931471805599454);

    /// The tightest interval enclosing $\log_{10} 2$.
    pub const LOG10_2: Self = const_interval!(0.30102999566398114, 0.3010299956639812);

    /// The tightest interval enclosing $\log_{10} \e$.
    pub const LOG10_E: Self = const_interval!(0.4342944819032518, 0.43429448190325187);

    /// The tightest interval enclosing $\log_2 10$.
    pub const LOG2_10: Self = const_interval!(3.321928094887362, 3.3219280948873626);

    /// The tightest interval enclosing $\log_2 \e$.
    pub const LOG2_E: Self = const_interval!(1.4426950408889634, 1.4426950408889636);

    /// The tightest interval enclosing $π$.
    pub const PI: Self = const_interval!(3.141592653589793, 3.1415926535897936);

    /// The tightest interval enclosing $\sqrt{2}$.
    pub const SQRT_2: Self = const_interval!(1.414213562373095, 1.4142135623730951);

    /// The tightest interval enclosing $2 π$.
    pub const TAU: Self = const_interval!(6.283185307179586, 6.283185307179587);
}