//! Mathematical and geometrical helpers with focus on ergonomics.
//! Note, this library is a work in progress.
//! Documentation is therefore not adequate.

#[macro_use]
mod macros;

mod bounds;
mod grid;
mod point;
mod size;

pub use crate::bounds::*;
pub use crate::grid::*;
pub use crate::point::*;

#[doc(inline)]
pub use crate::macros::*;

#[doc(inline)]
pub use crate::size::*;
