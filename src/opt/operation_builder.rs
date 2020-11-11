//! Operation implementation.

use crate::opt::Operation;
use arctk::{
    err::Error,
    file::{Build, Load},
};
use arctk_attr::input;
use ndarray::Array3;
use std::path::{Path, PathBuf};

/// Possible operation enumeration.
#[input]
#[derive(Clone)]
pub enum OperationBuilder {
    /// Generate a zero cube of the giver resolution.
    Zero([usize; 3]),
    /// Generate a unit cube of the giver resolution.
    Unit([usize; 3]),
    /// Sum cubes together.
    Sum(Vec<PathBuf>),
    /// Add a value to the data cube.
    Add(PathBuf, f64),
    /// Subtract a value from the data cube.
    Sub(PathBuf, f64),
    /// Multiply the datacube by the value.
    Mult(PathBuf, f64),
    /// Divide the datacube by the value.
    Div(PathBuf, f64),
    /// Normalise a data cube.
    Norm(PathBuf),
}

impl Build for OperationBuilder {
    type Inst = Operation;

    /// Build a usable instance.
    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Zero(res) => Self::Inst::Zero(res),
            Self::Unit(res) => Self::Inst::Unit(res),
            Self::Sum(data) => {
                let mut cubes = Vec::with_capacity(data.len());
                for d in &data {
                    cubes.push(Array3::load(&in_dir.join(d))?);
                }
                Self::Inst::Sum(cubes)
            }
            Self::Add(data, x) => {
                let cube = Array3::load(&in_dir.join(data))?;
                Self::Inst::Add(cube, x)
            }
            Self::Sub(data, x) => {
                let cube = Array3::load(&in_dir.join(data))?;
                Self::Inst::Sub(cube, x)
            }
            Self::Mult(data, x) => {
                let cube = Array3::load(&in_dir.join(data))?;
                Self::Inst::Mult(cube, x)
            }
            Self::Div(data, x) => {
                let cube = Array3::load(&in_dir.join(data))?;
                Self::Inst::Div(cube, x)
            }
            Self::Norm(data) => {
                let cube = Array3::load(&in_dir.join(data))?;
                Self::Inst::Norm(cube)
            }
        })
    }
}
