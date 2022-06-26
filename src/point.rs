macro_rules! impl_point {
    ($t: ty) => {
        impl Point2D<$t> {
            pub fn dot<P: IntoPoint2D<$t>>(&self, point: P) -> $t {
                let a: [$t; 2] = (*self).into();
                let b: [$t; 2] = point.to_point().into();

                (0..2).fold(<$t>::default(), |acc, i| acc + a[i] * b[i])
            }

            pub fn x(&self) -> $t {
                self.x
            }

            pub fn y(&self) -> $t {
                self.y
            }
        }

        impl std::ops::Add for Point2D<$t> {
            type Output = Point2D<$t>;

            fn add(self, rhs: Self) -> Self::Output {
                Point2D::new(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl std::ops::Sub for Point2D<$t> {
            type Output = Point2D<$t>;

            fn sub(self, rhs: Self) -> Self::Output {
                Point2D::new(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl_into_point!($t);
    };
}

macro_rules! impl_into_point {
    ($t: ty) => {
        impl IntoPoint2D<$t> for ($t, $t) {
            fn to_point(self) -> Point2D<$t> {
                Point2D::new(self.0, self.1)
            }
        }

        impl IntoPoint2D<$t> for [$t; 2] {
            fn to_point(self) -> Point2D<$t> {
                Point2D::new(self[0], self[1])
            }
        }
    };
}

/// A vector describing a point in 2D space.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point2D<T> {
    x: T,
    y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn from<P: IntoPoint2D<T>>(point: P) -> Self {
        point.to_point()
    }
}

impl_point!(u8);
impl_point!(u16);
impl_point!(u32);
impl_point!(u64);
impl_point!(u128);

impl_point!(i8);
impl_point!(i16);
impl_point!(i32);
impl_point!(i64);
impl_point!(i128);

impl_point!(f32);
impl_point!(f64);

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

/// Can be turned into a [Point2D]
pub trait IntoPoint2D<T> {
    fn to_point(self) -> Point2D<T>;
}

impl<T> IntoPoint2D<T> for Point2D<T> {
    fn to_point(self) -> Point2D<T> {
        self
    }
}
