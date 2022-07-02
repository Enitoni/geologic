//! Mathematical and geometrical helpers.
//! Note, this library is a work in progress.
//! Documentation is therefore not adequate.

#[macro_use]
mod macros;

mod bounds;
mod grid;
mod point;
mod size;

#[doc(hidden)]
mod prelude {
    pub use crate::bounds::*;
    pub use crate::grid::*;
    pub use crate::macros::*;
    pub use crate::point::*;
    pub use crate::size::*;
}

pub use prelude::*;
