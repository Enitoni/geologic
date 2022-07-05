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
