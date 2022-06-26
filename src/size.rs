macro_rules! impl_size {
    ($t: ty) => {
        impl Size2D<$t> {
            pub fn area(&self) -> $t {
                self.width * self.height
            }

            pub fn width(&self) -> $t {
                self.width
            }

            pub fn height(&self) -> $t {
                self.height
            }
        }

        impl std::ops::Add for Size2D<$t> {
            type Output = Size2D<$t>;

            fn add(self, rhs: Self) -> Self::Output {
                Size2D::new(self.width + rhs.width, self.height + rhs.height)
            }
        }

        impl std::ops::Sub for Size2D<$t> {
            type Output = Size2D<$t>;

            fn sub(self, rhs: Self) -> Self::Output {
                Size2D::new(self.width - rhs.width, self.height - rhs.height)
            }
        }

        impl_into_size!($t);
    };
}

macro_rules! impl_into_size {
    ($t: ty) => {
        impl IntoSize2D<$t> for ($t, $t) {
            fn to_size(self) -> Size2D<$t> {
                Size2D::new(self.0, self.1)
            }
        }

        impl IntoSize2D<$t> for [$t; 2] {
            fn to_size(self) -> Size2D<$t> {
                Size2D::new(self[0], self[1])
            }
        }
    };
}

/// A vector describing a two-dimensional size.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Size2D<T> {
    width: T,
    height: T,
}

impl<T> Size2D<T>
where
    T: Copy + Sized,
{
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    pub fn from<P: IntoSize2D<T>>(size: P) -> Self {
        size.to_size()
    }

    pub fn square(size: T) -> Self {
        Self::new(size, size)
    }
}

impl_size!(u8);
impl_size!(u16);
impl_size!(u32);
impl_size!(u64);
impl_size!(u128);

impl_size!(i8);
impl_size!(i16);
impl_size!(i32);
impl_size!(i64);
impl_size!(i128);

impl_size!(f32);
impl_size!(f64);

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

/// Can be turned into a [Size2D]
pub trait IntoSize2D<T> {
    fn to_size(self) -> Size2D<T>;
}

impl<T> IntoSize2D<T> for Size2D<T> {
    fn to_size(self) -> Size2D<T> {
        self
    }
}
