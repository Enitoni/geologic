use crate::{IntoPoint2D, IntoSize2D, Point2D, Size2D};

macro_rules! impl_bounds {
    ($t: ty) => {
        impl Bounds2D<$t> {
            pub fn expand<S: IntoSize2D<$t>>(&self, size: S) -> Bounds2D<$t> {
                let size = size.to_size();

                let new_width = self.size.width.max(size.width);
                let new_height = self.size.height.max(size.height);

                Bounds2D::from(self.position, (new_width, new_height))
            }

            pub fn shrink<S: IntoSize2D<$t>>(&self, size: S) -> Bounds2D<$t> {
                let size = size.to_size();

                let new_width = self.size.width.min(size.width);
                let new_height = self.size.height.min(size.height);

                Bounds2D::from(self.position, (new_width, new_height))
            }

            pub fn top(&self) -> $t {
                self.position.y
            }

            pub fn left(&self) -> $t {
                self.position.x
            }

            pub fn right(&self) -> $t {
                self.position.x + self.size.width
            }

            pub fn bottom(&self) -> $t {
                self.position.y + self.size.height
            }
        }

        impl IntoBounds2D<$t> for ($t, $t, $t, $t) {
            fn to_bounds(self) -> Bounds2D<$t> {
                let (x, y, width, height) = self;
                Bounds2D::new(x, y, width, height)
            }
        }

        impl IntoBounds2D<$t> for [$t; 4] {
            fn to_bounds(self) -> Bounds2D<$t> {
                let [x, y, width, height] = self;
                Bounds2D::new(x, y, width, height)
            }
        }
    };
}

/// A bounding box, with a size and position.
/// Also known as a rect, or rectangle.
#[derive(Debug, Clone, Copy)]
pub struct Bounds2D<T> {
    position: Point2D<T>,
    size: Size2D<T>,
}

impl<T> Bounds2D<T>
where
    T: Copy + Sized,
{
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        let position = Point2D::new(x, y);
        let size = Size2D::new(width, height);

        Self { position, size }
    }

    pub fn from<P, S>(position: P, size: S) -> Self
    where
        P: IntoPoint2D<T>,
        S: IntoSize2D<T>,
    {
        let position = position.to_point();
        let size = size.to_size();

        Self { position, size }
    }

    pub fn move_to<P: IntoPoint2D<T>>(&self, point: P) -> Bounds2D<T> {
        Bounds2D::from(point, self.size)
    }

    pub fn width(&self) -> T {
        self.size.width()
    }

    pub fn height(&self) -> T {
        self.size.height()
    }

    pub fn size(&self) -> Size2D<T> {
        self.size.clone()
    }

    pub fn position(&self) -> Point2D<T> {
        self.position.clone()
    }
}

impl_bounds!(usize);

impl_bounds!(u8);
impl_bounds!(u16);
impl_bounds!(u32);
impl_bounds!(u64);
impl_bounds!(u128);

impl_bounds!(i8);
impl_bounds!(i16);
impl_bounds!(i32);
impl_bounds!(i64);
impl_bounds!(i128);

impl_bounds!(f32);
impl_bounds!(f64);

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
