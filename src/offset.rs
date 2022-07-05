use std::ops::{Add, Mul, Sub};

use crate::{ToVector2D, Vector2D};

/// Marker struct for a vector used as a translation or velocity.
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Offset;

/// A two-dimensional vector representing an offset.
pub type Offset2D<T> = Vector2D<T, Offset>;

impl<T, Rhs> Add<Rhs> for Offset2D<T>
where
    Rhs: ToOffset2D<T>,
    T: Add<Output = T>,
{
    type Output = Offset2D<T>;

    fn add(self, rhs: Rhs) -> Self::Output {
        self.add_components(rhs)
    }
}

impl<T, Rhs> Sub<Rhs> for Offset2D<T>
where
    Rhs: ToOffset2D<T>,
    T: Sub<Output = T>,
{
    type Output = Offset2D<T>;

    fn sub(self, rhs: Rhs) -> Self::Output {
        self.sub_components(rhs)
    }
}

impl<T> Mul<T> for Offset2D<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Offset2D<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Offset2D::new(self.x * rhs, self.y * rhs)
    }
}

/// Trait alias for [ToVector2D] where `Kind` is [Offset].
pub trait ToOffset2D<T>: ToVector2D<T, Offset> {}
impl<T, V: ToVector2D<T, Offset>> ToOffset2D<T> for V {}
