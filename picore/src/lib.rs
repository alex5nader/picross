#![deny(missing_docs)]

//! Core library for picross frontends.

mod board;
mod puzzle;

pub use board::Board;
pub use puzzle::{Constraint, ConstraintEntry, ConstraintGroup, Puzzle};
