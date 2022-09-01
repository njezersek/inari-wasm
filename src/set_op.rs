use crate::{interval::*};

impl Interval {
    /// Returns $\hull(\self ∪ \rhs)$, the tightest interval that contains both `self` and `rhs` as its subsets.
    ///
    /// |                    | $\rhs = ∅$ | $\rhs = \[c, d\]$                      |
    /// | :----------------: | :--------: | :------------------------------------: |
    /// | $\self = ∅$        | $∅$        | $\[c, d\]$                             |
    /// | $\self = \[a, b\]$ | $\[a, b\]$ | $\[\min \set{a, c}, \max \set{b, d}\]$ |
    #[must_use]
    pub fn convex_hull(self, rhs: Self) -> Self {
        if self.is_empty() {
            return rhs;
        }
        if rhs.is_empty() {
            return self;
        }

        // [min(a, c), max(b, d)]
        Self { inf: f64::min(self.inf, rhs.inf), sup: f64::max(self.sup, rhs.sup) }
    }

    /// Returns $\self ∩ \rhs$, the intersection of `self` and `rhs`.
    ///
    /// |                    | $\rhs = ∅$ | $\rhs = \[c, d\]$                      |
    /// | :----------------: | :--------: | :------------------------------------: |
    /// | $\self = ∅$        | $∅$        | $∅$                                    |
    /// | $\self = \[a, b\]$ | $∅$        | $\[\max \set{a, c}, \min \set{b, d}\]$ |
    #[must_use]
    pub fn intersection(self, rhs: Self) -> Self {
        if self.either_empty(rhs) {
            return Self::EMPTY;
        }

        // [max(a, c), min(b, d)]
        let i = Self {
			inf: f64::max(self.inf, rhs.inf),
			sup: f64::min(self.sup, rhs.sup),
        };

        if i.inf_raw() > i.sup_raw() {
            Self::EMPTY
        } else {
            i
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use Interval as I;

    #[test]
    fn empty() {
        assert_eq!(I::EMPTY.convex_hull(I::PI), I::PI);
        assert_eq!(I::PI.convex_hull(I::EMPTY), I::PI);

        assert!(I::EMPTY.intersection(I::PI).is_empty());
        assert!(I::PI.intersection(I::EMPTY).is_empty());
    }
}
