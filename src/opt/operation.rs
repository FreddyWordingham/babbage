//! Operation implementation.

use arctk_attr::input;

/// Possible operation enumeration.
#[input]
#[derive(Clone)]
pub enum Operation {
    /// Generate a unit cube of the giver resolution.
    Unit([usize; 3]),
}

impl Operation {}
