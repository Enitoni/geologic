use num_traits::{AsPrimitive, Num, NumAssign};
use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

/// A vector describing a two-dimensional size.
#[derive(Debug, Default, PartialEq, Clone, Copy, Hash)]
pub struct Size2D<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size2D<T>
where
    T: Copy,
{
    /// Create a new [Size2D]. In most cases you should use
    /// the `size!()` macro instead.
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// let size = Size2D::new(20, 50);
    ///
    /// // Prefer doing this instead
    /// let size = size!(20, 50);
    /// ```
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    /// Returns a new [Size2D] where `width` and `height` are equal.
    ///
    /// Prefer using the splat syntax with the `size()` macro instead
    /// of calling this directly.
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// // This is acceptable, but...
    /// let size = Size2D::square(200);
    ///
    /// // ...this is the preferred way
    /// let size = size!(200; 2);
    ///
    /// assert_eq!(size, size!(200, 200));
    /// ```
    pub fn square(size: T) -> Self {
        Self::new(size, size)
    }

    /// Gets the area of the size. This is a shorthand for `size.width() * size.height()`
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// let size = size!(100, 200);
    ///
    /// assert_eq!(size.area(), 20000);
    /// ```
    pub fn area(&self) -> T
    where
        T: Mul<Output = T>,
    {
        self.width * self.height
    }

    /// Casts `self` into a new [`Size2D<C>`](crate::Size2D)
    /// where `C` is the (usually inferred) input type.
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// let a = size!(200.24, 400.90);
    /// let b: Size2D<u32> = a.cast();
    ///
    /// assert_eq!(b, size!(200, 400));
    /// ```
    pub fn cast<C>(&self) -> Size2D<C>
    where
        C: Copy + 'static,
        T: AsPrimitive<C>,
    {
        Size2D {
            width: self.width.as_(),
            height: self.height.as_(),
        }
    }
}

impl<T> Size2D<T>
where
    T: Num + Copy + PartialOrd,
{
    /// Compares the width and height components in `size` and `self`, creating a new size
    /// where the components are the greater values. This is the opposite of [`shrink()`](crate::Size2D::shrink)
    ///
    /// You can think of this as a one-way operation of expanding a rectangle.
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// let inner = size!(500, 700);
    /// let outer = size!(400, 900);
    ///
    /// assert_eq!(inner.grow(outer), size!(500, 900));
    /// ```
    pub fn grow<S: ToSize2D<T>>(&self, size: S) -> Size2D<T> {
        let size = size.to_size();

        let bigger_width = if size.width > self.width {
            size.width
        } else {
            self.width
        };

        let bigger_height = if size.height > self.height {
            size.height
        } else {
            self.height
        };

        Size2D::new(bigger_width, bigger_height)
    }

    /// Compares the width and height components in `size` and `self`, creating a new size
    /// where the components are the lesser values. This is the opposite of [`grow()`](crate::Size2D::grow)
    ///
    /// You can think of this as a one-way operation of collapsing a rectangle.
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// let outer = size!(500, 200);
    /// let inner = size!(100, 300);
    ///
    /// assert_eq!(outer.shrink(inner), size!(100, 200));
    /// ```
    pub fn shrink<S: ToSize2D<T>>(&self, size: S) -> Size2D<T> {
        let size = size.to_size();

        let smaller_width = if size.width < self.width {
            size.width
        } else {
            self.width
        };

        let smaller_height = if size.height < self.height {
            size.height
        } else {
            self.height
        };

        Size2D::new(smaller_width, smaller_height)
    }

    /// Returns a new size constrained within the `min` and `max`.
    /// This is a shorthand for `self.shrink(max).grow(min)`
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// let min = size!(200, 300);
    /// let max = size!(500, 500);
    ///
    /// assert_eq!(size!(100, 400).constrain(min, max), size!(200, 400));
    /// assert_eq!(size!(600, 300).constrain(min, max), size!(500, 300));
    /// ```
    pub fn constrain<S: ToSize2D<T>>(&self, min: S, max: S) -> Size2D<T> {
        let min = min.to_size();
        let max = max.to_size();

        self.shrink(max).grow(min)
    }
}

impl<T> Size2D<T>
where
    T: Copy,
    Self: PartialOrd,
{
    /// Returns the bigger size area between `self` and `size`.
    /// If you need to max individual components, use [`grow()`](crate::Size2D::grow)
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// let a = size!(200, 400);
    /// let b = size!(100, 300);
    ///
    /// assert_eq!(a.max_area(b), size!(200, 400));
    /// ```
    pub fn max_area<S: ToSize2D<T>>(&self, size: S) -> Size2D<T> {
        let this = *self;
        let size = size.to_size();

        if size > this {
            size
        } else {
            this
        }
    }

    /// Returns the smaller size area between `self` and `size`.
    /// If you need to min individual components, use [`shrink()`](crate::Size2D::shrink)
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// let a = size!(100, 300);
    /// let b = size!(200, 600);
    ///
    /// assert_eq!(a.min_area(b), size!(100, 300));
    /// ```
    pub fn min_area<S: ToSize2D<T>>(&self, size: S) -> Size2D<T> {
        let this = *self;
        let size = size.to_size();

        if size < this {
            size
        } else {
            this
        }
    }

    /// Clamps the size area between `min` and `max`.
    /// If you need to clamp individual components, use [`constrain()`](crate::Size2D::constrain)
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// let a = size!(200, 600);
    /// let b = size!(100, 200);
    ///
    /// assert_eq!(a.clamp_area(b, size!(300, 300)), size!(300, 300));
    /// ```
    pub fn clamp_area<S: ToSize2D<T>>(&self, min: S, max: S) -> Size2D<T> {
        let min = min.to_size();
        let max = max.to_size();

        self.min_area(max).max_area(min)
    }
}

/// Implements adding two sizes together.
impl<T, R> Add<R> for Size2D<T>
where
    T: Num + Copy,
    R: ToSize2D<T>,
{
    type Output = Size2D<T>;

    fn add(self, rhs: R) -> Self::Output {
        let rhs = rhs.to_size();
        Size2D::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl<T, R> AddAssign<R> for Size2D<T>
where
    T: Num + NumAssign + Copy,
    R: ToSize2D<T>,
{
    fn add_assign(&mut self, rhs: R) {
        let rhs = rhs.to_size();

        self.width += rhs.width;
        self.height += rhs.height;
    }
}

/// Implements subtracting two sizes
impl<T, R> Sub<R> for Size2D<T>
where
    T: Num + Copy,
    R: ToSize2D<T>,
{
    type Output = Size2D<T>;

    fn sub(self, rhs: R) -> Self::Output {
        let rhs = rhs.to_size();
        Size2D::new(self.width - rhs.width, self.height - rhs.height)
    }
}

impl<T, R> SubAssign<R> for Size2D<T>
where
    T: Num + NumAssign + Copy,
    R: ToSize2D<T>,
{
    fn sub_assign(&mut self, rhs: R) {
        let rhs = rhs.to_size();

        self.width -= rhs.width;
        self.height -= rhs.height;
    }
}

/// Implements multiplying two points
impl<T, R> Mul<R> for Size2D<T>
where
    T: Num + Copy,
    R: ToSize2D<T>,
{
    type Output = Size2D<T>;

    fn mul(self, rhs: R) -> Self::Output {
        let rhs = rhs.to_size();
        Size2D::new(self.width * rhs.width, self.height * rhs.height)
    }
}

impl<T, R> MulAssign<R> for Size2D<T>
where
    T: Num + NumAssign + Copy,
    R: ToSize2D<T>,
{
    fn mul_assign(&mut self, rhs: R) {
        let rhs = rhs.to_size();

        self.width *= rhs.width;
        self.height *= rhs.height;
    }
}

/// Implements dividing two points
impl<T, R> Div<R> for Size2D<T>
where
    T: Num + Copy,
    R: ToSize2D<T>,
{
    type Output = Size2D<T>;

    fn div(self, rhs: R) -> Self::Output {
        let rhs = rhs.to_size();
        Size2D::new(self.width / rhs.width, self.height / rhs.height)
    }
}

impl<T, R> DivAssign<R> for Size2D<T>
where
    T: Num + NumAssign + Copy,
    R: ToSize2D<T>,
{
    fn div_assign(&mut self, rhs: R) {
        let rhs = rhs.to_size();

        self.width /= rhs.width;
        self.height /= rhs.height;
    }
}

impl<T> PartialOrd for Size2D<T>
where
    T: Num + Copy + PartialEq + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let area = self.area();
        let other = other.area();

        if area == other {
            return Some(Ordering::Equal);
        }

        if area > other {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl<T> From<Size2D<T>> for (T, T) {
    fn from(size: Size2D<T>) -> Self {
        (size.width, size.height)
    }
}

impl<T> From<(T, T)> for Size2D<T>
where
    T: Num + Copy,
{
    fn from(tuple: (T, T)) -> Self {
        Size2D::new(tuple.0, tuple.1)
    }
}

/// A trait to aid in the ergonomics of creating a [Size2D]
/// and usage of interfaces expecting [Size2D].
///
/// # Examples
/// ```
/// # use geologic::*;
/// #
/// let size = size!(200, 400);
///
/// // We can pass another Size2D to this function
/// size.grow(size!(800, 400));
///
/// // But we can also pass a tuple
/// size.grow((400, 200));
/// ```
pub trait ToSize2D<T> {
    /// Creates a new [Size2D] from `self`
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// let size = (200, 100).to_size();
    ///
    /// assert_eq!(size, size!(200, 100));
    /// ```
    fn to_size(self) -> Size2D<T>;
}

impl<T> ToSize2D<T> for Size2D<T> {
    fn to_size(self) -> Size2D<T> {
        self
    }
}

// Allows passing a tuple to functions that expect IntoSize2D
impl<T> ToSize2D<T> for (T, T)
where
    T: Num + Copy,
{
    fn to_size(self) -> Size2D<T> {
        Size2D::new(self.0, self.1)
    }
}
