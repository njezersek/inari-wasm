use crate::{interval::*, classify::*, const_interval};

impl Interval {
    /// Rounds `self` to the closest integer toward $+∞$.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain | Range |
    /// | ------ | ----- |
    /// | $\R$   | $\Z$  |
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(0.2, 1.2).ceil(), const_interval!(1.0, 2.0));
    /// assert_eq!(const_interval!(0.8, 1.8).ceil(), const_interval!(1.0, 2.0));
    /// assert_eq!(const_interval!(-1.2, -0.2).ceil(), const_interval!(-1.0, 0.0));
    /// assert_eq!(const_interval!(-1.8, -0.8).ceil(), const_interval!(-1.0, 0.0));
    /// assert_eq!(Interval::EMPTY.ceil(), Interval::EMPTY);
    /// assert_eq!(Interval::ENTIRE.ceil(), Interval::ENTIRE);
    /// ```
    ///
    /// See also: [`Interval::floor`], [`Interval::trunc`].
    #[must_use]
    pub fn ceil(self) -> Self {
        // _mm_ceil_pd/_mm_floor_pd are slow, better to avoid shuffling them.
        // ceil([a, b]) = [ceil(a), ceil(b)]
        Self { inf: self.inf.ceil(), sup: self.sup.ceil() }
    }

    /// Rounds `self` to the closest integer toward $-∞$.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain | Range |
    /// | ------ | ----- |
    /// | $\R$   | $\Z$  |
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(0.2, 1.2).floor(), const_interval!(0.0, 1.0));
    /// assert_eq!(const_interval!(0.8, 1.8).floor(), const_interval!(0.0, 1.0));
    /// assert_eq!(const_interval!(-1.2, -0.2).floor(), const_interval!(-2.0, -1.0));
    /// assert_eq!(const_interval!(-1.8, -0.8).floor(), const_interval!(-2.0, -1.0));
    /// assert_eq!(Interval::EMPTY.floor(), Interval::EMPTY);
    /// assert_eq!(Interval::ENTIRE.floor(), Interval::ENTIRE);
    /// ```
    ///
    /// See also: [`Interval::ceil`], [`Interval::trunc`].
    #[must_use]
    pub fn floor(self) -> Self {
        // floor([a, b]) = [floor(a), floor(b)]
		Self { inf: self.inf.floor(), sup: self.sup.floor() }
	}

    /// Rounds `self` to the closest integer, away from zero in case of ties.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain | Range |
    /// | ------ | ----- |
    /// | $\R$   | $\Z$  |
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(0.2, 1.2).round(), const_interval!(0.0, 1.0));
    /// assert_eq!(const_interval!(0.5, 1.5).round(), const_interval!(1.0, 2.0));
    /// assert_eq!(const_interval!(0.8, 1.8).round(), const_interval!(1.0, 2.0));
    /// assert_eq!(const_interval!(-1.2, -0.2).round(), const_interval!(-1.0, 0.0));
    /// assert_eq!(const_interval!(-1.5, -0.5).round(), const_interval!(-2.0, -1.0));
    /// assert_eq!(const_interval!(-1.8, -0.8).round(), const_interval!(-2.0, -1.0));
    /// assert_eq!(Interval::EMPTY.round(), Interval::EMPTY);
    /// assert_eq!(Interval::ENTIRE.round(), Interval::ENTIRE);
    /// ```
    ///
    /// See also: [`Interval::round_ties_to_even`].
    #[must_use]
    pub fn round(self) -> Self {
        Self {
            inf: self.inf.round(),
			sup: self.sup.round(),
        }
    }

    /// Rounds `self` to the closest integer, the even number in case of ties.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain | Range |
    /// | ------ | ----- |
    /// | $\R$   | $\Z$  |
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(0.2, 1.2).round_ties_to_even(), const_interval!(0.0, 1.0));
    /// assert_eq!(const_interval!(0.5, 1.5).round_ties_to_even(), const_interval!(0.0, 2.0));
    /// assert_eq!(const_interval!(0.8, 1.8).round_ties_to_even(), const_interval!(1.0, 2.0));
    /// assert_eq!(const_interval!(-1.2, -0.2).round_ties_to_even(), const_interval!(-1.0, 0.0));
    /// assert_eq!(const_interval!(-1.5, -0.5).round_ties_to_even(), const_interval!(-2.0, 0.0));
    /// assert_eq!(const_interval!(-1.8, -0.8).round_ties_to_even(), const_interval!(-2.0, -1.0));
    /// assert_eq!(Interval::EMPTY.round_ties_to_even(), Interval::EMPTY);
    /// assert_eq!(Interval::ENTIRE.round_ties_to_even(), Interval::ENTIRE);
    /// ```
    ///
    /// See also: [`Interval::round`].
    #[must_use]
    pub fn round_ties_to_even(self) -> Self {
        self.round()
    }

    /// Returns the sign of `self`.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain | Range            |
    /// | ------ | ---------------- |
    /// | $\R$   | $\set{-1, 0, 1}$ |
    ///
    /// Note the difference in definition between [`f64::signum`] and this function;
    /// `+0.0_f64.signum()` and `-0.0_f64.signum()` return `+1.0` and `-1.0`, respectively,
    /// while the sign of zero is just zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(-10.0, -0.1).sign(), const_interval!(-1.0, -1.0));
    /// assert_eq!(const_interval!(0.0, 0.0).sign(), const_interval!(0.0, 0.0));
    /// assert_eq!(const_interval!(0.1, 10.0).sign(), const_interval!(1.0, 1.0));
    /// assert_eq!(Interval::EMPTY.sign(), Interval::EMPTY);
    /// assert_eq!(Interval::ENTIRE.sign(), const_interval!(-1.0, 1.0));
    /// ```
    #[must_use]
    pub fn sign(self) -> Self {
        match self.classify(){
			IntervalClass::E => Self::EMPTY,
			IntervalClass::M => const_interval!(-1.0, 1.0),
			IntervalClass::N0 | IntervalClass::N1 => const_interval!(-1.0, -1.0),
			IntervalClass::P0 | IntervalClass::P1 => const_interval!(1.0, 1.0),
			IntervalClass::Z => const_interval!(0.0, 0.0)
        }
    }

    /// Rounds `self` to the closest integer toward zero.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain | Range |
    /// | ------ | ----- |
    /// | $\R$   | $\Z$  |
    ///
    /// # Examples
    ///
    /// ```
    /// use inari::*;
    /// assert_eq!(const_interval!(0.2, 1.2).trunc(), const_interval!(0.0, 1.0));
    /// assert_eq!(const_interval!(0.8, 1.8).trunc(), const_interval!(0.0, 1.0));
    /// assert_eq!(const_interval!(-1.2, -0.2).trunc(), const_interval!(-1.0, 0.0));
    /// assert_eq!(const_interval!(-1.8, -0.8).trunc(), const_interval!(-1.0, 0.0));
    /// assert_eq!(Interval::EMPTY.trunc(), Interval::EMPTY);
    /// assert_eq!(Interval::ENTIRE.trunc(), Interval::ENTIRE);
    /// ```
    ///
    /// See also: [`Interval::ceil`], [`Interval::floor`].
    #[must_use]
    pub fn trunc(self) -> Self {
        Self {
            inf: self.inf.trunc(),
			sup: self.sup.trunc()
        }
    }
}