/// Shorthand for creating a new point.
/// The number of arguments determine the dimensions.
#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr) => {
        crate::Point2D::new($x, $y)
    };
}
