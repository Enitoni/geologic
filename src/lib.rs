//! Mathematical and geometrical helpers with focus on ergonomics.
//! Note, this library is a work in progress, documentation is therefore not adequate.

#[macro_use]
mod macros;

mod bounds;
mod grid;
mod point;
mod size;

pub use crate::bounds::*;
pub use crate::grid::*;
pub use crate::macros::*;
pub use crate::point::*;
pub use crate::size::*;
