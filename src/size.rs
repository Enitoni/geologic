use num_traits::{AsPrimitive, Num, NumAssign};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// A vector describing a two-dimensional size.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Size2D<T> {
    pub(crate) width: T,
    pub(crate) height: T,
}

impl<T> Size2D<T>
where
    T: Copy,
{
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    pub fn from<P: IntoSize2D<T>>(size: P) -> Self {
        size.into_size()
    }

    pub fn square(size: T) -> Self {
        Self::new(size, size)
    }

    pub fn width(&self) -> T {
        self.width
    }

    pub fn height(&self) -> T {
        self.height
    }

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
    T: Num + Copy,
{
    pub fn area(&self) -> T {
        self.width * self.height
    }
}

/// Implements adding two sizes together.
impl<T, R> Add<R> for Size2D<T>
where
    T: Num + Copy,
    R: IntoSize2D<T>,
{
    type Output = Size2D<T>;

    fn add(self, rhs: R) -> Self::Output {
        let rhs = rhs.into_size();
        Size2D::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl<T, R> AddAssign<R> for Size2D<T>
where
    T: Num + NumAssign + Copy,
    R: IntoSize2D<T>,
{
    fn add_assign(&mut self, rhs: R) {
        let rhs = rhs.into_size();

        self.width += rhs.width;
        self.height += rhs.height;
    }
}

/// Implements subtracting two sizes
impl<T, R> Sub<R> for Size2D<T>
where
    T: Num + Copy,
    R: IntoSize2D<T>,
{
    type Output = Size2D<T>;

    fn sub(self, rhs: R) -> Self::Output {
        let rhs = rhs.into_size();
        Size2D::new(self.width - rhs.width, self.height - rhs.height)
    }
}

impl<T, R> SubAssign<R> for Size2D<T>
where
    T: Num + NumAssign + Copy,
    R: IntoSize2D<T>,
{
    fn sub_assign(&mut self, rhs: R) {
        let rhs = rhs.into_size();

        self.width -= rhs.width;
        self.height -= rhs.height;
    }
}

/// Implements multiplying two points
impl<T, R> Mul<R> for Size2D<T>
where
    T: Num + Copy,
    R: IntoSize2D<T>,
{
    type Output = Size2D<T>;

    fn mul(self, rhs: R) -> Self::Output {
        let rhs = rhs.into_size();
        Size2D::new(self.width * rhs.width, self.height * rhs.height)
    }
}

impl<T, R> MulAssign<R> for Size2D<T>
where
    T: Num + NumAssign + Copy,
    R: IntoSize2D<T>,
{
    fn mul_assign(&mut self, rhs: R) {
        let rhs = rhs.into_size();

        self.width *= rhs.width;
        self.height *= rhs.height;
    }
}

/// Implements dividing two points
impl<T, R> Div<R> for Size2D<T>
where
    T: Num + Copy,
    R: IntoSize2D<T>,
{
    type Output = Size2D<T>;

    fn div(self, rhs: R) -> Self::Output {
        let rhs = rhs.into_size();
        Size2D::new(self.width / rhs.width, self.height / rhs.height)
    }
}

impl<T, R> DivAssign<R> for Size2D<T>
where
    T: Num + NumAssign + Copy,
    R: IntoSize2D<T>,
{
    fn div_assign(&mut self, rhs: R) {
        let rhs = rhs.into_size();

        self.width /= rhs.width;
        self.height /= rhs.height;
    }
}

impl<T> From<Size2D<T>> for [T; 2] {
    fn from(size: Size2D<T>) -> Self {
        [size.width, size.height]
    }
}

impl<T> From<Size2D<T>> for (T, T) {
    fn from(size: Size2D<T>) -> Self {
        (size.width, size.height)
    }
}

impl<T> From<[T; 2]> for Size2D<T>
where
    T: Num + Copy,
{
    fn from(arr: [T; 2]) -> Self {
        Size2D::from(arr)
    }
}

impl<T> From<(T, T)> for Size2D<T>
where
    T: Num + Copy,
{
    fn from(tuple: (T, T)) -> Self {
        Size2D::from(tuple)
    }
}

/// Can be turned into a [Size2D]
pub trait IntoSize2D<T> {
    fn into_size(self) -> Size2D<T>;
}

impl<T> IntoSize2D<T> for Size2D<T> {
    fn into_size(self) -> Size2D<T> {
        self
    }
}

// Allows passing a tuple to functions that expect IntoSize2D
impl<T> IntoSize2D<T> for (T, T)
where
    T: Num + Copy,
{
    fn into_size(self) -> Size2D<T> {
        Size2D::new(self.0, self.1)
    }
}

// Allows passing an array to functions that expect IntoSize2D
impl<T> IntoSize2D<T> for [T; 2]
where
    T: Num + Copy,
{
    fn into_size(self) -> Size2D<T> {
        Size2D::new(self[0], self[1])
    }
}
