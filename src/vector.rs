use std::{
    marker::PhantomData,
    ops::{Add, Mul, Sub},
};

use num_traits::{AsPrimitive, Signed};

/// A trait defining common helper methods
/// to aid in the usage of a vector, or types with underlying vectors.
pub trait Vector<T, ToVector> {
    fn dot(&self, rhs: ToVector) -> T
    where
        T: Copy + Mul<Output = T> + Add<Output = T>;

    fn cross(&self, rhs: ToVector) -> T
    where
        T: Copy + Mul<Output = T> + Sub<Output = T>;

    fn distance(&self, rhs: ToVector) -> T
    where
        T: Signed + Copy + Add<Output = T> + Sub<Output = T>;
}

/// A generic vector with an X and Y component.
#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
pub struct Vector2D<T, Kind> {
    pub x: T,
    pub y: T,

    _kind: PhantomData<Kind>,
}

impl<T, K> Vector2D<T, K> {
    /// Returns a new [Vector2D] with `x` and `y` components.
    ///
    /// In most cases you should not call this directly, but rather use
    /// the macros to get the specialized variants.
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// // Avoid doing this
    /// let point: Vector2D<_, Point> = Vector2D::new(20, 40);
    ///
    /// // This is better, but not great
    /// let point: Point2D<_> = Vector2D::new(20, 40);
    ///
    /// // This is acceptable, but...
    /// let point = Point2D::new(20, 40);
    ///
    /// // ...this is the preferred way
    /// let point = point!(20, 40);
    /// ```
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
            _kind: PhantomData,
        }
    }

    /// Returns a new [Vector2D] where both components are set to `value`.
    ///
    /// Prefer using the splat syntax with the specialized macros instead of
    /// calling this directly.
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// // This is acceptable, but...
    /// let offset = Offset2D::splat(5);
    ///
    /// // ...this is the preferred way
    /// let offset = offset!(5; 2);
    ///
    /// assert_eq!(offset, offset!(5, 5));
    /// ```
    pub fn splat(value: T) -> Self
    where
        T: Copy,
    {
        Self {
            x: value,
            y: value,
            _kind: PhantomData,
        }
    }

    /// Casts `self` into a new [Vector2D]
    /// where components are the (usually inferred) input type.
    pub fn cast<C>(self) -> Vector2D<C, K>
    where
        C: Copy + 'static,
        T: AsPrimitive<C>,
    {
        Vector2D {
            x: self.x.as_(),
            y: self.y.as_(),
            _kind: PhantomData,
        }
    }

    #[doc(hidden)]
    pub(crate) fn add_components<V: ToVector2D<T, U>, U>(self, rhs: V) -> Self
    where
        T: Add<Output = T>,
    {
        let rhs = rhs.to_vector();
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }

    #[doc(hidden)]
    pub(crate) fn sub_components<V: ToVector2D<T, U>, U>(self, rhs: V) -> Self
    where
        T: Sub<Output = T>,
    {
        let rhs = rhs.to_vector();
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T, K, ToVector> Vector<T, ToVector> for Vector2D<T, K>
where
    ToVector: ToVector2D<T, K>,
{
    /// Returns the dot product of `self` and `rhs`.
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// let a = offset!(5, 10);
    /// let b = offset!(10, 5);
    ///
    /// assert_eq!(a.dot(b), 100);
    /// ```
    fn dot(&self, rhs: ToVector) -> T
    where
        T: Copy + Mul<Output = T> + Add<Output = T>,
    {
        let rhs = rhs.to_vector();

        let a = self.x * rhs.x;
        let b = self.y * rhs.y;

        a + b
    }

    /// Returns the normal of the cross product between `self` and `rhs`.
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// let a = offset!(0, 0);
    /// let b = offset!(20, 20);
    ///
    /// assert_eq!(a.cross(b), 0);
    /// ```
    fn cross(&self, rhs: ToVector) -> T
    where
        T: Copy + Mul<Output = T> + Sub<Output = T>,
    {
        let rhs = rhs.to_vector();
        self.x * rhs.y - self.y * rhs.x
    }

    /// Returns the absolute distance between `self` and `rhs`.
    ///
    /// # Examples
    /// ```
    /// # use geologic::*;
    /// #
    /// let a = offset!(10, 10);
    /// let b = offset!(0, 0);
    ///
    /// assert_eq!(a.distance(b), 20);
    /// ```
    fn distance(&self, rhs: ToVector) -> T
    where
        T: Signed + Copy + Add<Output = T> + Sub<Output = T>,
    {
        let rhs = rhs.to_vector();
        (rhs.x - self.x).abs() + (rhs.y - self.y).abs()
    }
}

/// A helper trait to aid with the ergonomics of using a [`Vector2D`].
pub trait ToVector2D<T, K> {
    /// Converts this type into a [`Vector2D`].
    fn to_vector(self) -> Vector2D<T, K>;
}

/// Makes it so that [`Vector2D`] itself can be used for interfaces expecting it.
impl<T, K> ToVector2D<T, K> for Vector2D<T, K> {
    fn to_vector(self) -> Vector2D<T, K> {
        self
    }
}

/// Makes it so a tuple can be used for interfaces expecting [`Vector2D`].
impl<T, K> ToVector2D<T, K> for (T, T) {
    fn to_vector(self) -> Vector2D<T, K> {
        Vector2D::new(self.0, self.1)
    }
}

impl<T, K> From<Vector2D<T, K>> for (T, T) {
    fn from(vector: Vector2D<T, K>) -> Self {
        (vector.x, vector.y)
    }
}

impl<T, K> From<(T, T)> for Vector2D<T, K> {
    fn from(tuple: (T, T)) -> Self {
        Vector2D::new(tuple.0, tuple.1)
    }
}
