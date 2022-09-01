use crate::interval::*;

// NOTE: `eq` is implemented in interval.rs

impl Interval{
	/// Returns `true` if `rhs` is a member of `self`: $\rhs ∈ \self$.
    ///
    /// The result is `false` whenever `rhs` is infinite or NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use inari_wasm::*;
    /// assert!(const_interval!(1.0, 2.0).contains(1.0));
    /// assert!(!Interval::EMPTY.contains(1.0));
    /// assert!(Interval::ENTIRE.contains(1.0));
    /// ```
    ///
    /// $±∞$ and NaN are not real numbers, thus do not belong to any interval:
    ///
    /// ```
    /// use inari_wasm::*;
    /// assert!(!Interval::ENTIRE.contains(f64::INFINITY));
    /// assert!(!Interval::ENTIRE.contains(f64::NEG_INFINITY));
    /// assert!(!Interval::ENTIRE.contains(f64::NAN));
    /// ```
    pub fn contains(self, rhs: f64) -> bool {
        rhs.is_finite() & {
            // a ≤ c  ∧  c ≤ b
            self.inf <= rhs && rhs <= self.sup
        }
    }

    /// Returns `true` if `self` and `rhs` are disjoint:
    ///
    /// $$
    /// \self ∩ \rhs = ∅,
    /// $$
    ///
    /// or equivalently,
    ///
    /// $$
    /// ∀x ∈ \self, ∀y ∈ \rhs : x ≠ y,
    /// $$
    ///
    /// or equivalently,
    ///
    /// |                    | $\rhs = ∅$ | $\rhs = \[c, d\]$ |
    /// | :----------------: | :--------: | :---------------: |
    /// | $\self = ∅$        | `true`     | `true`            |
    /// | $\self = \[a, b\]$ | `true`     | $b < c ∨ d < a$   |
    ///
    /// # Examples
    ///
    /// ```
    /// use inari_wasm::*;
    /// assert!(const_interval!(1.0, 2.0).disjoint(const_interval!(3.0, 4.0)));
    /// assert!(!const_interval!(1.0, 3.0).disjoint(const_interval!(3.0, 4.0)));
    /// assert!(!const_interval!(1.0, 5.0).disjoint(const_interval!(3.0, 4.0)));
    /// assert!(Interval::EMPTY.disjoint(Interval::EMPTY));
    /// assert!(Interval::EMPTY.disjoint(Interval::ENTIRE));
    /// ```
    pub fn disjoint(self, rhs: Self) -> bool {
        self.either_empty(rhs) | {
            // b < c  ∨  d < a
            self.sup < rhs.inf || rhs.sup < self.inf
        }
    }

    /// Returns `true` if `self` is interior to `rhs`:
    ///
    /// $$
    /// (∀x ∈ \self, ∃y ∈ \rhs : x < y) ∧ (∀x ∈ \self, ∃y ∈ \rhs : y < x),
    /// $$
    ///
    /// or equivalently,
    ///
    /// |                    | $\rhs = ∅$ | $\rhs = \[c, d\]$ |
    /// | :----------------: | :--------: | :---------------: |
    /// | $\self = ∅$        | `true`     | `true`            |
    /// | $\self = \[a, b\]$ | `false`    | $c <′ a ∧ b <′ d$ |
    ///
    /// where $<′$ is defined as:
    ///
    /// $$
    /// x <′ y :⟺ x < y ∨ x = y = -∞ ∨ x = y = +∞.
    /// $$
    ///
    /// # Examples
    ///
    /// ```
    /// use inari_wasm::*;
    /// assert!(const_interval!(1.1, 1.9).interior(const_interval!(1.0, 2.0)));
    /// assert!(!const_interval!(1.1, 2.0).interior(const_interval!(1.0, 2.0)));
    /// assert!(Interval::EMPTY.interior(Interval::EMPTY));
    /// assert!(Interval::ENTIRE.interior(Interval::ENTIRE));
    /// ```
    pub fn interior(self, rhs: Self) -> bool {
        // self = ∅  ∨  b < d  ∨  b = d = +∞
        let l = self.is_empty()
            || self.sup < rhs.sup
            || self.sup == f64::INFINITY && rhs.sup == f64::INFINITY; //all(eq(shuffle13(self.rep, rhs.rep), splat(f64::INFINITY)));
        // rhs = ∅  ∨  c < a  ∨  a = c = -∞
        let r = self.is_empty()
            || rhs.inf < self.inf
            || rhs.inf == f64::NEG_INFINITY && self.inf == f64::NEG_INFINITY; // all(eq(shuffle02(self.rep, rhs.rep), splat(f64::INFINITY)));
        l && r
    }

    /// Returns `true` if `self` is nonempty and bounded.
    ///
    /// # Examples
    ///
    /// ```
    /// use inari_wasm::*;
    /// assert!(const_interval!(1.0, 2.0).is_common_interval());
    /// assert!(!const_interval!(1.0, f64::INFINITY).is_common_interval());
    /// assert!(!Interval::EMPTY.is_common_interval());
    /// assert!(!Interval::ENTIRE.is_common_interval());
    /// ```
    pub fn is_common_interval(self) -> bool {
        // -∞ < a  ∧  b < +∞
        f64::NEG_INFINITY < self.inf && self.sup < f64::INFINITY // all(lt(self.rep, splat(f64::INFINITY)))
    }


    /// Returns `true` if `self` is empty: $\self = ∅$.
    ///
    /// # Examples
    ///
    /// ```
    /// use inari_wasm::*;
    /// assert!(!const_interval!(1.0, 1.0).is_empty());
    /// assert!(Interval::EMPTY.is_empty());
    /// assert!(!Interval::ENTIRE.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.inf.is_nan()
    }

    /// Returns `true` if $\self = \[-∞, +∞\]$.
    ///
    /// # Examples
    ///
    /// ```
    /// use inari_wasm::*;
    /// assert!(!const_interval!(1.0, f64::INFINITY).is_entire());
    /// assert!(!Interval::EMPTY.is_entire());
    /// assert!(Interval::ENTIRE.is_entire());
    /// ```
    pub fn is_entire(self) -> bool {
        self.inf == f64::NEG_INFINITY && self.sup == f64::INFINITY // all(eq(self.rep, splat(f64::INFINITY)))
    }


    /// Returns `true` if `self` consists of a single real number:
    ///
    /// $$
    /// ∃x ∈ ℝ : \self = \[x, x\].
    /// $$
    ///
    /// The result is `false` whenever `self` is empty or unbounded.
    ///
    /// # Examples
    ///
    /// ```
    /// use inari_wasm::*;
    /// assert!(const_interval!(1.0, 1.0).is_singleton());
    /// assert!(!const_interval!(1.0, 2.0).is_singleton());
    /// assert!(!Interval::EMPTY.is_singleton());
    /// assert!(!Interval::ENTIRE.is_singleton());
    /// ```
    ///
    /// 0.1 is not representable as a [`f64`] number:
    ///
    /// ```
    /// use inari_wasm::*;
    /// // The singleton interval that consists of the closest [`f64`] number to 0.1.
    /// assert!(const_interval!(0.1, 0.1).is_singleton());
    /// // The tightest interval that encloses 0.1.
    /// #[cfg(feature = "gmp")]
    /// assert!(!interval!("[0.1, 0.1]").unwrap().is_singleton());
    /// ```
    pub fn is_singleton(self) -> bool {
        // a = d
        self.inf == self.sup
    }

    /// Returns `true` if `self` is weakly less than `rhs`:
    ///
    /// $$
    /// (∀x ∈ \self, ∃y ∈ \rhs : x ≤ y) ∧ (∀y ∈ \rhs, ∃x ∈ \self : x ≤ y),
    /// $$
    ///
    /// or equivalently,
    ///
    /// |                    | $\rhs = ∅$ | $\rhs = \[c, d\]$ |
    /// | :----------------: | :--------: | :---------------: |
    /// | $\self = ∅$        | `true`     | `false`           |
    /// | $\self = \[a, b\]$ | `false`    | $a ≤ c ∧ b ≤ d$   |
    ///
    /// # Examples
    ///
    /// ```
    /// use inari_wasm::*;
    /// assert!(const_interval!(1.0, 2.0).less(const_interval!(3.0, 4.0)));
    /// assert!(const_interval!(1.0, 3.0).less(const_interval!(2.0, 4.0)));
    /// assert!(const_interval!(1.0, 4.0).less(const_interval!(1.0, 4.0)));
    /// assert!(Interval::EMPTY.less(Interval::EMPTY));
    /// assert!(!Interval::EMPTY.less(Interval::ENTIRE));
    /// assert!(!Interval::ENTIRE.less(Interval::EMPTY));
    /// assert!(Interval::ENTIRE.less(Interval::ENTIRE));
    /// ```
    pub fn less(self, rhs: Self) -> bool {
        // self = ∅  ∨  b ≤ d
        let l = self.is_empty() || self.sup <= rhs.sup;
        // rhs = ∅  ∨  a ≤ c
        let r = rhs.is_empty() || self.inf <= rhs.inf;
        l && r
    }

    /// Returns `true` if `self` is to the left of `rhs` but may touch it:
    ///
    /// $$
    /// ∀x ∈ \self, ∀y ∈ \rhs : x ≤ y,
    /// $$
    ///
    /// or equivalently,
    ///
    /// |                    | $\rhs = ∅$ | $\rhs = \[c, d\]$ |
    /// | :----------------: | :--------: | :---------------: |
    /// | $\self = ∅$        | `true`     | `true`            |
    /// | $\self = \[a, b\]$ | `true`     | $b ≤ c$           |
    ///
    /// # Examples
    ///
    /// ```
    /// use inari_wasm::*;
    /// assert!(const_interval!(1.0, 2.0).precedes(const_interval!(3.0, 4.0)));
    /// assert!(const_interval!(1.0, 3.0).precedes(const_interval!(3.0, 4.0)));
    /// assert!(!const_interval!(1.0, 3.0).precedes(const_interval!(2.0, 4.0)));
    /// assert!(Interval::EMPTY.precedes(Interval::EMPTY));
    /// assert!(Interval::EMPTY.precedes(Interval::ENTIRE));
    /// assert!(Interval::ENTIRE.precedes(Interval::EMPTY));
    /// assert!(!Interval::ENTIRE.precedes(Interval::ENTIRE));
    /// ```
    pub fn precedes(self, rhs: Self) -> bool {
        // self = ∅  ∨  rhs = ∅  ∨  b ≤ c
        self.either_empty(rhs) | (self.sup <= rhs.inf)
    }

    /// Returns `true` if `self` is strictly less than `rhs`:
    ///
    /// $$
    /// (∀x ∈ \self, ∃y ∈ \rhs : x < y) ∧ (∀y ∈ \self, ∃x ∈ \rhs : x < y),
    /// $$
    ///
    /// or equivalently,
    ///
    /// |                    | $\rhs = ∅$ | $\rhs = \[c, d\]$ |
    /// | :----------------: | :--------: | :---------------: |
    /// | $\self = ∅$        | `true`     | `false`           |
    /// | $\self = \[a, b\]$ | `false`    | $a <′ c ∧ b <′ d$ |
    ///
    /// where $<′$ is defined as:
    ///
    /// $$
    /// x <′ y :⟺ x < y ∨ x = y = -∞ ∨ x = y = +∞.
    /// $$
    ///
    /// # Examples
    ///
    /// ```
    /// use inari_wasm::*;
    /// assert!(const_interval!(1.0, 2.0).strict_less(const_interval!(3.0, 4.0)));
    /// assert!(const_interval!(1.0, 3.0).strict_less(const_interval!(2.0, 4.0)));
    /// assert!(!const_interval!(1.0, 4.0).strict_less(const_interval!(2.0, 4.0)));
    /// assert!(const_interval!(1.0, f64::INFINITY).strict_less(const_interval!(2.0, f64::INFINITY)));
    /// assert!(Interval::EMPTY.strict_less(Interval::EMPTY));
    /// assert!(!Interval::EMPTY.strict_less(Interval::ENTIRE));
    /// assert!(!Interval::ENTIRE.strict_less(Interval::EMPTY));
    /// assert!(Interval::ENTIRE.strict_less(Interval::ENTIRE));
    /// ```
    pub fn strict_less(self, rhs: Self) -> bool {
        // self = ∅  ∨  b < d  ∨  b = d = +∞
        let l = self.is_empty()
            || self.sup < rhs.sup
            || self.sup == f64::INFINITY && rhs.sup == f64::INFINITY; // all(eq(shuffle13(self.rep, rhs.rep), splat(f64::INFINITY)));
        // rhs = ∅  ∨  a < c  ∨  a = c = -∞
        let r = rhs.is_empty()
            || self.inf < rhs.inf
            || self.inf == f64::NEG_INFINITY && rhs.inf == f64::NEG_INFINITY; // all(eq(shuffle02(self.rep, rhs.rep), splat(f64::INFINITY)));
        l && r
    }

    /// Returns `true` if `self` is strictly to the left of `rhs`:
    ///
    /// $$
    /// ∀x ∈ \self, ∀y ∈ \rhs : x < y,
    /// $$
    ///
    /// or equivalently,
    ///
    /// |                    | $\rhs = ∅$ | $\rhs = \[c, d\]$ |
    /// | :----------------: | :--------: | :---------------: |
    /// | $\self = ∅$        | `true`     | `true`            |
    /// | $\self = \[a, b\]$ | `true`     | $b < c$           |
    pub fn strict_precedes(self, rhs: Self) -> bool {
        // self = ∅  ∨  rhs = ∅  ∨  b < c
        self.either_empty(rhs) | (self.sup < rhs.inf)
    }

    /// Returns `true` if `self` is a subset of `rhs`:
    ///
    /// $$
    /// \self ⊆ \rhs,
    /// $$
    ///
    /// or equivalently,
    ///
    /// $$
    /// ∀x ∈ \self, ∃y ∈ \rhs : x = y,
    /// $$
    ///
    /// or equivalently,
    ///
    /// |                    | $\rhs = ∅$ | $\rhs = \[c, d\]$ |
    /// | :----------------: | :--------: | :---------------: |
    /// | $\self = ∅$        | `true`     | `true`            |
    /// | $\self = \[a, b\]$ | `false`    | $c ≤ a ∧ b ≤ d$   |
    ///
    /// # Examples
    ///
    /// ```
    /// use inari_wasm::*;
    /// assert!(const_interval!(1.0, 2.0).subset(const_interval!(1.0, 2.0)));
    /// assert!(Interval::EMPTY.subset(Interval::EMPTY));
    /// assert!(Interval::EMPTY.subset(Interval::ENTIRE));
    /// assert!(Interval::ENTIRE.subset(Interval::ENTIRE));
    /// ```
    pub fn subset(self, rhs: Self) -> bool {
        self.is_empty() | {
            // c ≤ a  ∧  b ≤ d
            rhs.inf <= self.inf && self.sup <= rhs.sup // all(le(self.rep, rhs.rep))
        }
    }

    pub(crate) fn both_empty(self, rhs: Self) -> bool {
        self.is_empty() & rhs.is_empty()
    }

    pub(crate) fn either_empty(self, rhs: Self) -> bool {
        self.is_empty() | rhs.is_empty()
    }
}