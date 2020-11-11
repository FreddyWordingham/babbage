//! Operation implementation.

use arctk_attr::input;
use ndarray::Array3;

/// Possible operation enumeration.
#[input]
#[derive(Clone)]
pub enum Operation {
    /// Generate a zero cube of the giver resolution.
    Zero([usize; 3]),
    /// Generate a unit cube of the giver resolution.
    Unit([usize; 3]),
    /// Add two cubes together.
    Add(Array3<f64>, Array3<f64>),
    /// Subtract the rhs from the lhs.
    Sub(Array3<f64>, Array3<f64>),
}

impl Operation {
    /// Perform the operation.
    #[inline]
    #[must_use]
    pub fn run(&self) -> Array3<f64> {
        match self {
            Self::Zero(res) => (Array3::zeros(*res)),
            Self::Unit(res) => (Array3::zeros(*res) + 1.0),
            Self::Add(lhs, rhs) => lhs + rhs,
            Self::Sub(lhs, rhs) => lhs - rhs,
        }
    }
}
