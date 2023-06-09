use crate::{classify::*, interval::*};

impl Interval {
    /// Returns the absolute value of `self`.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain | Range     |
    /// | ------ | --------- |
    /// | $\R$   | $\[0, ∞)$ |
    #[must_use]
    pub fn abs(self) -> Self {
		let (a, b) = (self.inf, self.sup);
		
        use IntervalClass::*;
        match self.classify() {
            E | P0 | P1 | Z => self,
            M => {
                // [0, max(-a, b)]
				Self { inf: 0.0, sup: f64::max(-a, b) }
            }
            N0 | N1 => {
                // [-b, -a]
				Self { inf: -b, sup: -a }
            }
        }
    }

    /// Returns the maximum of `self` and `rhs`.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain | Range |
    /// | ------ | ----- |
    /// | $\R^2$ | $\R$  |
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
		if self.either_empty(rhs) {
			return Self::EMPTY;
		}

		Self { inf: f64::max(self.inf, rhs.inf), sup: f64::max(self.sup, rhs.sup) }
    }

    /// Returns the minimum of `self` and `rhs`.
    ///
    /// The domain and the range of the point function are:
    ///
    /// | Domain | Range |
    /// | ------ | ----- |
    /// | $\R^2$ | $\R$  |
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
		if self.either_empty(rhs) {
			return Self::EMPTY;
		}

		Self { inf: f64::min(self.inf, rhs.inf), sup: f64::min(self.sup, rhs.sup) }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use Interval as I;

    #[test]
    fn empty() {
        assert!(I::EMPTY.abs().is_empty());

        assert!(I::EMPTY.max(I::PI).is_empty());
        assert!(I::PI.max(I::EMPTY).is_empty());

        assert!(I::EMPTY.min(I::PI).is_empty());
        assert!(I::PI.min(I::EMPTY).is_empty());
    }
}