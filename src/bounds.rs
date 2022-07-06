use std::ops::{Add, Mul, Sub};

use num_traits::Num;

use crate::{Offset2D, Point2D, Size2D, ToPoint2D, ToSize2D};

/// A two-dimensional bounding box.
#[derive(Default, Debug, PartialEq, Clone, Copy, Hash)]
pub struct Bounds2D<T> {
    position: Point2D<T>,
    size: Size2D<T>,
}

impl<T> Bounds2D<T>
where
    T: Copy,
{
    /// Creates a new [Bounds2D]. In most cases you should use
    /// the `bounds!()` macro instead.
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// let bounds = Bounds2D::new(20, 50, 80, 90);
    ///
    /// // Prefer doing this instead
    /// let bounds = bounds!(20, 50, 80, 90);
    /// ```
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        let position = Point2D::new(x, y);
        let size = Size2D::new(width, height);

        Self { position, size }
    }

    /// Returns a new [Bounds2D] where all components are set to `value`.
    ///
    /// Prefer using the splat syntax with the macro instead of
    /// calling this directly.
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// // This is acceptable, but...
    /// let bounds = Bounds2D::splat(8);
    ///
    /// // ...this is the preferred way
    /// let bounds = bounds!(8; 2);
    ///
    /// assert_eq!(bounds, bounds!(8, 8, 8, 8));
    /// ```
    pub fn splat(value: T) -> Self {
        Self::from_position_and_size(Point2D::splat(value), Size2D::square(value))
    }

    /// Creates a new [Bounds2D] from a position and size.
    /// This is useful when you have a size and position, and want to create a bounds out of it.
    ///
    /// However, if you already have a [Size2D] or a [Point2D],
    /// you should use the `.with_` method instead.
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// let bounds = Bounds2D::from_position_and_size(point!(20, 40), size!(10, 10));
    ///
    /// // Prefer doing this instead
    /// let bounds = point!(20, 40).with_size(size!(10, 10));
    /// ```
    pub fn from_position_and_size<P, S>(position: P, size: S) -> Self
    where
        P: ToPoint2D<T>,
        S: ToSize2D<T>,
    {
        let position = position.to_vector();
        let size = size.to_size();

        Self { position, size }
    }

    /// Creates a new [Bounds2D] with the specified position.
    pub fn with_position<P: ToPoint2D<T>>(&self, point: P) -> Bounds2D<T> {
        Bounds2D::from_position_and_size(point, self.size)
    }

    /// Creates a new [Bounds2D] with the specified size.
    pub fn with_size<S: ToSize2D<T>>(&self, size: S) -> Bounds2D<T> {
        Bounds2D::from_position_and_size(self.position, size)
    }

    pub fn width(&self) -> T {
        self.size.width
    }

    pub fn height(&self) -> T {
        self.size.height
    }

    pub fn top(&self) -> T {
        self.position.y
    }

    pub fn left(&self) -> T {
        self.position.x
    }

    pub fn right(&self) -> T
    where
        T: Add<Output = T>,
    {
        self.position.x + self.size.width
    }

    pub fn bottom(&self) -> T
    where
        T: Add<Output = T>,
    {
        self.position.y + self.size.height
    }

    pub fn area(&self) -> T
    where
        T: Mul<Output = T>,
    {
        self.size.area()
    }

    pub fn size(&self) -> Size2D<T> {
        self.size
    }

    pub fn position(&self) -> Point2D<T> {
        self.position
    }
}

impl<T> Bounds2D<T>
where
    T: Num + Copy + Ord,
{
    /// See [`Size2D::grow()`](crate::Size2D::grow) for more information.
    pub fn grow<S: ToSize2D<T>>(&self, size: S) -> Bounds2D<T> {
        Bounds2D::from_position_and_size(self.position, self.size.grow(size))
    }

    /// See [`Size2D::shrink()`](crate::Size2D::shrink) for more information.
    pub fn shrink<S: ToSize2D<T>>(&self, size: S) -> Bounds2D<T> {
        Bounds2D::from_position_and_size(self.position, self.size.shrink(size))
    }

    /// See [`Size2D::constrain()`](crate::Size2D::constrain) for more information.
    pub fn constrain<S: ToSize2D<T>>(&self, min: S, max: S) -> Bounds2D<T> {
        Bounds2D::from_position_and_size(self.position, self.size.constrain(min, max))
    }

    /// See [`Size2D::max_area()`](crate::Size2D::max_area) for more information.
    pub fn max_area<S: ToSize2D<T>>(&self, size: S) -> Bounds2D<T> {
        Bounds2D::from_position_and_size(self.position, self.size.max_area(size))
    }

    /// See [`Size2D::min_area()`](crate::Size2D::min_area) for more information.
    pub fn min_area<S: ToSize2D<T>>(&self, size: S) -> Bounds2D<T> {
        Bounds2D::from_position_and_size(self.position, self.size.min_area(size))
    }

    /// See [`Size2D::clamp_area()`](crate::Size2D::clamp_area) for more information.
    pub fn clamp_area<S: ToSize2D<T>>(&self, min: S, max: S) -> Bounds2D<T> {
        Bounds2D::from_position_and_size(self.position, self.size.clamp_area(min, max))
    }
}

impl<T> Add<Offset2D<T>> for Bounds2D<T>
where
    T: Num + Copy,
{
    type Output = Bounds2D<T>;

    fn add(self, rhs: Offset2D<T>) -> Self::Output {
        Bounds2D::from_position_and_size(self.position + rhs, self.size)
    }
}

impl<T> Add<Size2D<T>> for Bounds2D<T>
where
    T: Num + Copy,
{
    type Output = Bounds2D<T>;

    fn add(self, rhs: Size2D<T>) -> Self::Output {
        Bounds2D::from_position_and_size(self.position, self.size + rhs)
    }
}

impl<T> Sub<Offset2D<T>> for Bounds2D<T>
where
    T: Num + Copy,
{
    type Output = Bounds2D<T>;

    fn sub(self, rhs: Offset2D<T>) -> Self::Output {
        Bounds2D::from_position_and_size(self.position - rhs, self.size)
    }
}

impl<T> Sub<Size2D<T>> for Bounds2D<T>
where
    T: Num + Copy,
{
    type Output = Bounds2D<T>;

    fn sub(self, rhs: Size2D<T>) -> Self::Output {
        Bounds2D::from_position_and_size(self.position, self.size - rhs)
    }
}

impl<T> From<Bounds2D<T>> for [T; 4] {
    fn from(bounds: Bounds2D<T>) -> Self {
        [
            bounds.position.x,
            bounds.position.y,
            bounds.size.width,
            bounds.size.height,
        ]
    }
}

impl<T> From<Bounds2D<T>> for (T, T, T, T) {
    fn from(bounds: Bounds2D<T>) -> Self {
        (
            bounds.position.x,
            bounds.position.y,
            bounds.size.width,
            bounds.size.height,
        )
    }
}

pub trait IntoBounds2D<T> {
    fn to_bounds(self) -> Bounds2D<T>;
}

impl<T> IntoBounds2D<T> for Bounds2D<T> {
    fn to_bounds(self) -> Bounds2D<T> {
        self
    }
}

impl<T> IntoBounds2D<T> for (T, T, T, T)
where
    T: Num + Copy,
{
    fn to_bounds(self) -> Bounds2D<T> {
        let (x, y, width, height) = self;
        Bounds2D::new(x, y, width, height)
    }
}

impl<T> IntoBounds2D<T> for [T; 4]
where
    T: Num + Copy,
{
    fn to_bounds(self) -> Bounds2D<T> {
        let [x, y, width, height] = self;
        Bounds2D::new(x, y, width, height)
    }
}
