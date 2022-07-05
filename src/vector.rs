use std::marker::PhantomData;

/// A generic vector with an X and Y component.
///
/// In most cases you should not instantiate this directly, but rather use
/// the specialized macros.
#[derive(Default, Clone, Copy, Debug, PartialEq, Hash)]
pub struct Vector2D<T, Kind> {
    pub x: T,
    pub y: T,

    _kind: PhantomData<Kind>,
}

impl<T, K> Vector2D<T, K> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
            _kind: PhantomData,
        }
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
