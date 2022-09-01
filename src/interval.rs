
use std::{
    convert::TryFrom,
    error::Error,
    fmt,
    hash::{Hash, Hasher},
    result,
};


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntervalErrorKind {
    PossiblyUndefinedOperation,
    UndefinedOperation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntervalError {
    pub(crate) kind: IntervalErrorKind,
}

impl IntervalError {
    /// Returns the type of the error.
    pub fn kind(&self) -> IntervalErrorKind {
        self.kind
    }
}

impl fmt::Display for IntervalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            IntervalErrorKind::PossiblyUndefinedOperation => {
                write!(f, "possibly undefined operation")
            }
            IntervalErrorKind::UndefinedOperation => write!(f, "undefined operation"),
        }
    }
}

impl Error for IntervalError {}

/// An alias for [`Result<T, E>`](`result::Result`) with [`E = IntervalError`](`IntervalError`).
pub type Result<T> = result::Result<T, IntervalError>;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    // An interval is stored as a pair of f64
    //
    // - An nonempty interval [a, b] is stored as {inf: a, sup: b}.
    // - An empty interval is stored as {inf: NaN, sup: NaN}.
    //
    pub inf: f64,
    pub sup: f64,
}

impl Interval {
    pub(crate) fn inf_raw(self) -> f64 {
        self.inf
    }

    pub(crate) fn sup_raw(self) -> f64 {    
        self.sup
    } 

    pub(crate) fn with_infsup_raw(a: f64, b: f64) -> Self {
        Self {
            inf: a,
            sup: b,
        }
    }

    pub(crate) fn zero() -> Self {
        Self { inf: 0.0, sup: 0.0 }
    }
}

impl PartialEq for Interval {
    fn eq(&self, rhs: &Interval) -> bool {
        self.both_empty(*rhs) || (self.inf == rhs.inf && self.sup == rhs.sup)
    }
}

impl Eq for Interval {}

impl Hash for Interval {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inf.to_bits().hash(state);
        self.sup.to_bits().hash(state);
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.inf, self.sup)
    }
}


impl TryFrom<(f64, f64)> for Interval {
    type Error = IntervalError;

    fn try_from((a, b): (f64, f64)) -> Result<Self> {
        if a <= b && a != f64::INFINITY && b != f64::NEG_INFINITY {
            Ok(Self::with_infsup_raw(a, b))
        } else {
            Err(Self::Error {
                kind: IntervalErrorKind::UndefinedOperation,
            })
        }
    }
}


// macros

#[doc(hidden)]
#[macro_export]
macro_rules! _interval {
    ($a:expr, $b:expr) => {{
        use ::std::{convert::TryFrom, primitive::*};
        fn is_f64(_: f64) {}
        is_f64($a);
        is_f64($b);
        $crate::Interval::try_from(($a, $b))
    }};
}

#[macro_export]
macro_rules! interval {
    ($a:expr, $b:expr) => {
        $crate::_interval!($a, $b)
    };

    ($a:expr) => {
        $crate::_interval!($a, $a)
    };
}

/// Creates an [`Interval`] from [`f64`] bounds.
///
/// `a` and `b` must be constant `f64` values. The result is an [`Interval`].
///
/// The macro can be used in [constant expressions](https://doc.rust-lang.org/reference/const_eval.html#constant-expressions).
///
/// The usage is almost the same as the macro [`interval!(a, b)`](`interval!`)
/// except that this macro returns an [`Interval`] directly,
/// or results in a compilation error if the construction is invalid.
#[macro_export]
macro_rules! const_interval {
    ($a:expr, $b:expr) => {{
        const _: () = assert!($a <= $b && $a != f64::INFINITY && $b != f64::NEG_INFINITY);

        Interval{inf: $a, sup: $b}
    }};

    ($a:expr) => {
        const_interval!($a, $a)
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn main() {
        let x = const_interval!(-1.2, 1.2);
        let a = 
            const_interval!(2.0) * x.powi(5) +
            const_interval!(-4.0) * x.powi(3) +
            x + const_interval!(1.0);
        println!("{}", a);
    }

    #[test]
    pub fn benchmark(){
        let N = 1_000_000;
        let x = const_interval!(10.0, 20.0);
        let y = const_interval!(10.0, 20.0);
    
        let mut sum = 0.0;
        // time 
        let start = std::time::Instant::now();
        for _ in 0..N {
            let r = x.powi(2) + y.powi(2) + const_interval!(3.0) * (const_interval!(10.0) * x.powi(3)).sin() - const_interval!(1.0);
            sum += r.inf;
        }
        let duration = start.elapsed();
        println!("The loop took: {:?}", duration);
    }

    #[test]
    pub fn example(){
        let x = const_interval!(0.0, 2.0);
        let y = x.sin() + const_interval!(1.0);
        println!("{}", y); // [1, 2]
    }
}
