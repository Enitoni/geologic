use num_traits::{AsPrimitive, Num, NumAssign};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// A 2D vector. You can use this
/// for positioning or displacement.
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Point2D<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> Point2D<T>
where
    T: Copy,
{
    /// Create a new [Point2D] where `T` is X and Y.
    pub fn new(x: T, y: T) -> Self
    where
        T: Num,
    {
        Self { x, y }
    }

    pub fn from<P: IntoPoint2D<T>>(point: P) -> Self {
        point.into_point()
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    /// Casts self into a new `Point2D<C>` where
    /// `C` is the (usually inferred) input type.
    pub fn cast<C>(&self) -> Point2D<C>
    where
        C: Copy + 'static,
        T: AsPrimitive<C>,
    {
        Point2D {
            x: self.x.as_(),
            y: self.y.as_(),
        }
    }
}

impl<T> Point2D<T>
where
    T: Num + Copy,
{
    /// Returns a new Point2D with X and Y set to `value`
    pub fn splat(&self, value: T) -> Point2D<T> {
        Point2D::new(value, value)
    }

    /// Gets the dot product between self and `P`
    pub fn dot<P: IntoPoint2D<T>>(&self, point: P) -> T {
        let a: [T; 2] = (*self).into();
        let b: [T; 2] = point.into_point().into();

        (0..2).fold(<T>::zero(), |acc, i| acc + a[i] * b[i])
    }

    /// Gets the cross product of self and `P`
    pub fn cross<P: IntoPoint2D<T>>(&self, point: P) -> T {
        let point = point.into_point();
        self.x * point.y - point.x * self.y
    }
}

/// Implements adding two points together.
impl<T> Add for Point2D<T>
where
    T: Num + Copy,
{
    type Output = Point2D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> AddAssign for Point2D<T>
where
    T: NumAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

/// Implements subtracting two points
impl<T> Sub for Point2D<T>
where
    T: Num + Copy,
{
    type Output = Point2D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> SubAssign for Point2D<T>
where
    T: Num + NumAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

/// Implements multiplying two points
impl<T> Mul for Point2D<T>
where
    T: Num + Copy,
{
    type Output = Point2D<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Point2D::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T> MulAssign for Point2D<T>
where
    T: Num + NumAssign + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

/// Implements dividing two points
impl<T> Div for Point2D<T>
where
    T: Num + Copy,
{
    type Output = Point2D<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Point2D::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T> DivAssign for Point2D<T>
where
    T: Num + NumAssign + Copy,
{
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

// Ergonomics for converting between Point2D and other types
impl<T> From<Point2D<T>> for [T; 2] {
    fn from(point: Point2D<T>) -> Self {
        [point.x, point.y]
    }
}

impl<T> From<Point2D<T>> for (T, T) {
    fn from(point: Point2D<T>) -> Self {
        (point.x, point.y)
    }
}

impl<T> From<[T; 2]> for Point2D<T>
where
    T: Num + Copy,
{
    fn from(arr: [T; 2]) -> Self {
        Point2D::from(arr)
    }
}

impl<T> From<(T, T)> for Point2D<T>
where
    T: Num + Copy,
{
    fn from(tuple: (T, T)) -> Self {
        Point2D::from(tuple)
    }
}

// Can be turned into a [Point2D]
pub trait IntoPoint2D<T> {
    fn into_point(self) -> Point2D<T>;
}

// Allow passing Point2D to functions that expect IntoPoint2D
impl<T> IntoPoint2D<T> for Point2D<T> {
    fn into_point(self) -> Point2D<T> {
        self
    }
}

// Allows passing a tuple to functions that expect IntoPoint2D
impl<T> IntoPoint2D<T> for (T, T)
where
    T: Num + Copy,
{
    fn into_point(self) -> Point2D<T> {
        Point2D::new(self.0, self.1)
    }
}

// Allows passing an array to functions that expect IntoPoint2D
impl<T> IntoPoint2D<T> for [T; 2]
where
    T: Num + Copy,
{
    fn into_point(self) -> Point2D<T> {
        Point2D::new(self[0], self[1])
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn simple_math() {
        // Addition
        let a = point!(10, 20);
        let b = point!(20, 20);
        assert_eq!(a + b, point!(30, 40));

        // Addition assign
        let mut a = point!(20, 20);

        a += point!(10, 0);
        assert_eq!(a, point!(30, 20));

        // Subtraction
        let a = point!(10, 20);
        let b = point!(20, 20);
        assert_eq!(a - b, point!(-10, 0));

        // Subtraction assign
        let mut a = point!(20, 20);

        a -= point!(10, 0);
        assert_eq!(a, point!(10, 20));

        // Multiplication
        let a = point!(10, 0);
        let b = point!(20, 20);

        assert_eq!(a * b, point!(200, 0));

        // Multiplication assign
        let mut a = point!(20, 20);

        a *= point!(2, 0);
        assert_eq!(a, point!(40, 0));

        // Division
        let a = point!(10, 2);
        let b = point!(2, 2);

        assert_eq!(a / b, point!(5, 1));

        // Division assign
        let mut a = point!(20, 20);

        a /= point!(2, 2);
        assert_eq!(a, point!(10, 10));
    }
}
