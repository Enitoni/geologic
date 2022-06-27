use std::{
    fmt::{Debug, Display},
    ops::Range,
};

use crate::IntoBounds2D;

#[macro_export]
macro_rules! grid {
    ([$default: expr; $chunk_size: expr]: $width: literal x $height: literal) => {
        Grid2D::<_, { $width * $height * $chunk_size }>::new($default, $width, $chunk_size)
    };
    ([$default: expr; $chunk_size: expr]: $size: literal) => {
        grid!([$default; $chunk_size]: $size x $size)
    };
    ([$default: expr]: $size: literal) => {
        grid!([$default; 1]: $size x $size)
    };
}

/// A 2D array, representing a grid with X and Y space.
#[derive(Clone)]
pub struct Grid2D<T, const S: usize>
where
    T: Sized,
{
    pub(crate) arr: [T; S],
    width: usize,
    chunk_size: usize,
}

impl<T, const S: usize> Grid2D<T, S> {
    #[allow(dead_code)] // This is used by a macro, and is thus not unused
    pub(crate) fn new(default: T, width: usize, chunk_size: usize) -> Grid2D<T, S>
    where
        T: Sized + Copy,
    {
        debug_assert!(
            chunk_size > 0,
            "Grid2D chunk size must be greater than zero. Got: {}",
            chunk_size
        );

        Grid2D {
            arr: [default; S],
            chunk_size,
            width,
        }
    }

    /// Returns ranges for each row of the bounding box
    pub(crate) fn row_ranges<B: IntoBounds2D<usize>>(
        bounds: B,
        grid_width: usize,
        chunk_size: usize,
    ) -> impl Iterator<Item = Range<usize>> {
        let bounds = bounds.to_bounds();

        debug_assert!(
            bounds.size().area() > 0,
            "Bounding area must be greater than 0"
        );

        let (x, y, width, height) = bounds.into();

        let cell_x = x * chunk_size;

        let width = width * chunk_size;
        let full_width = grid_width * chunk_size;

        // This is the index of the first value in the starting cell
        let start_cell = cell_x + (y * full_width);

        (0..height).map(move |row| {
            let cell_y = row * full_width;

            let start = start_cell + cell_y;
            let end = start + width;

            start..end
        })
    }

    pub fn portion<B: IntoBounds2D<usize>>(&self, bounds: B) -> Vec<T>
    where
        T: Copy,
    {
        let rows = Self::row_ranges(bounds, self.width, self.chunk_size);

        rows.flat_map(|r| &self.arr[r]).map(|v| *v).collect()
    }

    pub fn values(&self) -> &[T] {
        &self.arr
    }

    pub fn write<B: IntoBounds2D<usize>, W: FnMut(usize, &mut [T])>(
        &mut self,
        bounds: B,
        mut write: W,
    ) {
        let rows = Self::row_ranges(bounds, self.width, self.chunk_size);
        let arr = &mut self.arr;

        for (row, range) in rows.enumerate() {
            let slice = &mut arr[range];
            write(row, slice);
        }
    }
}

impl<T, const S: usize> Display for Grid2D<T, S>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values: Vec<_> = self
            .arr
            .chunks_exact(self.chunk_size)
            .map(|chunk| {
                chunk.iter().fold(String::new(), |acc, s| {
                    (!acc.is_empty()).then(|| acc.clone() + ", ").unwrap_or(acc) + &s.to_string()
                }) + " | "
            })
            .collect();

        let rows: String = (&values)
            .chunks_exact(self.width)
            .map(|chunk| format!("| {}\n", chunk.iter().fold(String::new(), |acc, s| acc + s)))
            .collect();

        let height = (self.arr.len() / self.chunk_size) / self.width;

        write!(
            f,
            "Grid 2D ({w}x{h}; {}):\n{}",
            self.chunk_size,
            rows,
            w = self.width,
            h = height,
        )
    }
}

impl<T, const S: usize> Debug for Grid2D<T, S>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Grid2D<T, S> as Display>::fmt(&self, f)
    }
}

#[cfg(test)]
mod test {
    use super::Grid2D;
    use crate::Bounds2D;

    #[test]
    fn row_ranges() {
        let bounds = Bounds2D::from((1, 1), (1, 2));

        // 0, 1, 2
        // 3, X, 5,
        // 6, X, 8,
        let mut ranges = Grid2D::<u32, { 3 * 3 }>::row_ranges(bounds, 3, 1);

        assert_eq!(ranges.next(), Some(4..5));
        assert_eq!(ranges.next(), Some(7..8));

        // 0-1,   2-3,  4-5
        // 6-7,   X,    10-11,
        // 12-13, X,    16-17,
        let mut ranges = Grid2D::<u32, { 3 * 3 }>::row_ranges(bounds, 3, 2);

        // A chunk size of 2 doubles the step of the range
        assert_eq!(ranges.next(), Some(8..10));
        assert_eq!(ranges.next(), Some(14..16));
    }

    #[test]
    fn write() {
        let mut grid = grid!([0u32; 2]: 3);

        grid.write((0, 1, 2, 1), |_, slice| {
            for value in slice {
                *value += 5;
            }
        });

        grid.write((1, 1, 2, 2), |_, slice| {
            for value in slice {
                *value += 2;
            }
        });

        let expected_result = &[
            0, 0, 0, 0, 0, 0, //
            5, 5, 7, 7, 2, 2, //
            0, 0, 2, 2, 2, 2,
        ];

        assert_eq!(grid.values(), expected_result);
    }
}
