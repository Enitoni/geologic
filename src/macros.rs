//! Helper macros for instantiating different kinds of algebraic types.
//!
//! _The following documentation applies to:_
//! - `bounds!()`
//! - `size!()`
//! - `point!()`
//! - `offset!()`
//!
//! # Creating vector types
//! To create one of the aforementioned types, simply specify the components.
//! The number of components determine the dimensions, for example
//!
//! ```ignore
//! // This creates a Size2D, because we specified 2 arguments.
//! let very_big = size!(500, 500);
//! ```
//!
//! ## Exceptions
//! For [`Bounds2D`](crate::Bounds2D) it is the same,
//! except the number of arguments are doubled.
//!
//! ```ignore
//! // This creates a Bounds2D
//! let bounds = bounds!(0, 20, 10, 10);
//! ```
//!
//! # Splat syntax
//! A type can be created with initalized values
//! by specifying the value followed by a semicolon
//! and the number of dimensions.
//!
//! ```
//! # use geologic::*;
//! #
//! // This creates a Point2D where X and Y is `5`.
//! let point = point!(5; 2);
//!
//! // This creates an Offset2D where X and Y is `0u8`.
//! let offset = offset!(u8; 2);
//!
//! // No exception for bounds here, this is a Bounds2D.
//! let bounds = bounds!(10; 2);
//! ```

/// Creates a new size.
#[macro_export]
macro_rules! size {
    ($t: ty; 2) => {
        $crate::Size2D::square(<$t>::default());
    };
    ($v:expr; 2) => {
        $crate::Size2D::square($v)
    };
    ($width:expr, $height:expr) => {
        $crate::Size2D::new($width, $height)
    };
}

/// Creates a new bounding box.
#[macro_export]
macro_rules! bounds {
    ($t: ty; 2) => {
        $crate::Bounds2D::splat(<$t>::default());
    };
    ($v:expr; 2) => {
        $crate::Bounds2D::splat($v)
    };
    ($x:expr, $y:expr, $width:expr, $height:expr) => {
        $crate::Bounds2D::new($x, $y, $width, $height)
    };
}

/// Creates a new point vector.
#[macro_export]
macro_rules! point {
    ($($t:tt)*) => {{
        let point: $crate::Point2D<_> = $crate::__vector!($($t)*);
        point
    }};
}

/// Creates a new offset vector.
#[macro_export]
macro_rules! offset {
    ($($t:tt)*) => {{
        let offset: $crate::Offset2D<_> = $crate::__vector!($($t)*);
        offset
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __vector {
    ($t: ty; 2) => {
        $crate::Vector2D::splat(<$t>::default());
    };
    ($v:expr; 2) => {
        $crate::Vector2D::splat($v)
    };
    ($x:expr, $y:expr) => {
        $crate::Vector2D::new($x, $y)
    };
}
