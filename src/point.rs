use std::ops::{Add, Sub};

use crate::{Offset2D, ToOffset2D, ToVector2D, Vector2D};

/// Marker struct for a vector used as a point.
#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
pub struct Point;

/// A two-dimensional vector representing a point.
///
/// # Examples
/// ```
/// # use geologic::*;
/// #
/// let point = point!(10, 0);
///
/// // A point can be moved with an offset
/// let moved_point = point + offset!(20, 5);
/// assert_eq!(moved_point, point!(30, 5));
///
/// // A tuple offset can also be used
/// let moved_point = point + (20, 5);
/// assert_eq!(moved_point, point!(30, 5));
/// ```
pub type Point2D<T> = Vector2D<T, Point>;

impl<T> Point2D<T> {
    /// Returns the offset between `self` and `point`.
    ///
    /// Order matters here, so if you are trying to get the offset
    /// needed for point A to get to point B, you would do `a.offset(b)`.
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// let a = point!(10, 40);
    /// let b = point!(5, 60);
    ///
    /// assert_eq!(a.offset(b), offset!(-5, 20));
    /// ```
    pub fn offset<P: ToPoint2D<T>>(self, point: P) -> Offset2D<T>
    where
        T: Sub<Output = T>,
    {
        let point = point.to_vector();
        Offset2D::new(point.x - self.x, point.y - self.y)
    }
}

impl<T, Rhs> Add<Rhs> for Point2D<T>
where
    Rhs: ToOffset2D<T>,
    T: Add<Output = T>,
{
    type Output = Point2D<T>;

    fn add(self, rhs: Rhs) -> Self::Output {
        self.add_components(rhs)
    }
}

impl<T, Rhs> Sub<Rhs> for Point2D<T>
where
    Rhs: ToOffset2D<T>,
    T: Sub<Output = T>,
{
    type Output = Point2D<T>;

    fn sub(self, rhs: Rhs) -> Self::Output {
        self.sub_components(rhs)
    }
}

/// Trait alias for [ToVector2D] where `Kind` is [Point].
pub trait ToPoint2D<T>: ToVector2D<T, Point> {}
impl<T, V: ToVector2D<T, Point>> ToPoint2D<T> for V {}
