//! Operation implementation.

use ndarray::Array3;
use ndarray_stats::QuantileExt;

/// Possible operation enumeration.
pub enum Operation {
    /// Generate a zero cube of the giver resolution.
    Zero([usize; 3]),
    /// Generate a unit cube of the giver resolution.
    Unit([usize; 3]),
    /// Sum cubes together.
    Sum(Vec<Array3<f64>>),
    /// Add a value to the data cube.
    Add(Array3<f64>, f64),
    /// Subtract a value from the data cube.
    Sub(Array3<f64>, f64),
    /// Multiply the datacube by the value.
    Mult(Array3<f64>, f64),
    /// Divide the datacube by the value.
    Div(Array3<f64>, f64),
    /// Normalise a data cube.
    Norm(Array3<f64>),
}

impl Operation {
    /// Perform the operation.
    #[inline]
    #[must_use]
    pub fn run(&self) -> Array3<f64> {
        match self {
            Self::Zero(res) => (Array3::zeros(*res)),
            Self::Unit(res) => (Array3::zeros(*res) + 1.0),
            Self::Sum(data) => {
                let mut base = data[0].clone();
                for d in data.iter().skip(1) {
                    base += d;
                }
                base
            }
            Self::Add(data, x) => data + *x,
            Self::Sub(data, x) => data - *x,
            Self::Mult(data, x) => data * *x,
            Self::Div(data, x) => data / *x,
            Self::Norm(data) => {
                let max = *data.max().expect("Failed to determine maximal value.");
                data / max
            }
        }
    }
}
