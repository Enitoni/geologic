//! Mathematical and geometrical abstractions with a focus on ergonomics.
//!
//! ## Disclaimer
//! This library is a work in progress still,
//! there may be breaking changes between versions.
//!
//! If you want a missing feature
//! or want to report a bug, please create an issue on GitHub.
//!
//! # Examples
//! ```
//! // Import the prelude for convenience
//! use geologic::*;
//!
//! let position = point!(0, 40);
//! let size = size!(5; 2);
//!
//! // Derive a Bounds2D from a position and size.
//! let bounds = position.with_size(size);
//!
//! // Translate the bounds with an offset
//! let moved_bounds = bounds + offset!(3, 5);
//!
//! // Resize the bounds up with a size
//! let enlarged_bounds = moved_bounds + size!(10, 10);
//! assert_eq!(enlarged_bounds, bounds!(3, 45, 15, 15));
//!
//! // We can also use tuples for operations like these
//! let moved_bounds = bounds + (10, 20);
//! assert_eq!(moved_bounds, bounds!(10, 60, 5, 5))
//! ```

#[macro_use]
pub mod macros;

mod bounds;
mod grid;
mod offset;
mod point;
mod size;
mod vector;

pub use crate::bounds::*;
pub use crate::grid::*;
pub use crate::macros::*;
pub use crate::offset::*;
pub use crate::point::*;
pub use crate::size::*;
pub use crate::vector::*;
