/// Shorthand for creating a new point.
/// The number of arguments determine the dimensions.
#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr) => {
        crate::Point2D::new($x, $y)
    };
}

/// Shorthand for creating a new size.
/// The number of arguments determine the dimensions.
///
/// # Examples
/// ```
/// # use geologic::*;
/// // A Size2D
/// let size = size!(200, 200);
/// ```
#[macro_export]
macro_rules! size {
    ($width:expr, $height:expr) => {
        $crate::Size2D::new($width, $height)
    };
}

/// Shorthand for creating a new bounding box.
/// The number of arguments determine the dimensions.
///
/// # Examples
/// ```
/// # use geologic::*;
/// // A Bounds2D
/// let bounds = bounds!(20, 50, 800, 900);
/// ```
#[macro_export]
macro_rules! bounds {
    ($x:expr, $y:expr, $width:expr, $height:expr) => {
        $crate::Bounds2D::new($x, $y, $width, $height)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __vector {
    ($v:expr; 2) => {
        $crate::Vector2D::splat($v)
    };
    ($x:expr, $y:expr) => {
        $crate::Vector2D::new($x, $y)
    };
}

/// Creates a new position vector.
/// The number of arguments determine the dimensions.
///
/// # Examples
/// ```
/// # use geologic::*;
/// #
/// // A two-dimensional position
/// let position = position!(20, 40);
///
/// // A two-dimensional splatted position
/// let position = position!(20; 2);
/// ```
#[macro_export]
macro_rules! position {
    ($($t:tt)*) => {{
        let position: $crate::Position2D<_> = $crate::__vector!($($t)*);
        position
    }};
}

/// Creates a new offset vector.
/// The number of arguments determine the dimensions.
///
/// # Examples
/// ```
/// # use geologic::*;
/// #
/// // A two-dimensional offset
/// let offset = offset!(20, 40);
///
/// // A two-dimensional splatted offset
/// let offset = offset!(20; 2);
/// ```
#[macro_export]
macro_rules! offset {
    ($($t:tt)*) => {{
        let offset: $crate::Offset2D<_> = $crate::__vector!($($t)*);
        offset
    }};
}
