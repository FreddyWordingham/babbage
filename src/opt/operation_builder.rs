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
    /// Add two cubes together.
    Add(PathBuf, PathBuf),
}

impl Build for OperationBuilder {
    type Inst = Operation;

    /// Build a usable instance.
    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Zero(res) => Self::Inst::Zero(res),
            Self::Unit(res) => Self::Inst::Unit(res),
            Self::Add(lhs, rhs) => {
                let l = Array3::load(&in_dir.join(lhs)).expect("Failed to load lhs value array.");
                let r = Array3::load(&in_dir.join(rhs)).expect("Failed to load rhs value array.");
                Self::Inst::Add(l, r)
            }
        })
    }
}
