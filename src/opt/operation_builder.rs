//! Operation implementation.

use crate::opt::Operation;
use arctk::{err::Error, file::Build};
use arctk_attr::input;
use std::path::Path;

/// Possible operation enumeration.
#[input]
#[derive(Clone)]
pub enum OperationBuilder {
    /// Generate a unit cube of the giver resolution.
    Unit([usize; 3]),
}

impl Build for OperationBuilder {
    type Inst = Operation;

    /// Build a usable instance.
    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Unit(res) => Self::Inst::Unit(res),
        })
    }
}
