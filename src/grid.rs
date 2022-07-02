use std::{
    fmt::{Debug, Display},
    ops::{IndexMut, Range},
};

use crate::{IntoBounds2D, IntoPoint2D, Size2D};

fn index_at<P: IntoPoint2D<usize>>(point: P, grid_width: usize, chunk_size: usize) -> usize {
    let (x, y) = point.into_point().into();

    let cell_x = x * chunk_size;
    let cell_y = y * (grid_width * chunk_size);

    cell_x + cell_y
}

fn row_ranges<B: IntoBounds2D<usize>>(
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

pub trait Interpretation2D:
    AsRef<[Self::Item]> + IndexMut<Range<usize>, Output = [Self::Item]>
{
    type Item;

    fn write<W>(&mut self, rows: impl Iterator<Item = Range<usize>>, mut write: W)
    where
        W: FnMut(usize, &mut [Self::Item]),
    {
        for (row, range) in rows.enumerate() {
            let slice = &mut self[range];
            write(row, slice);
        }
    }

    fn values(&self) -> &[Self::Item] {
        self.as_ref()
    }

    fn size(&self) -> usize;
}

impl<T, const C: usize> Interpretation2D for [T; C] {
    type Item = T;

    fn values(&self) -> &[Self::Item] {
        self
    }

    fn size(&self) -> usize {
        self.len()
    }
}

impl<T> Interpretation2D for [T] {
    type Item = T;

    fn values(&self) -> &[Self::Item] {
        self
    }

    fn size(&self) -> usize {
        self.len()
    }
}

impl<T> Interpretation2D for Vec<T> {
    type Item = T;

    fn values(&self) -> &[Self::Item] {
        self
    }

    fn size(&self) -> usize {
        self.len()
    }
}

/// A 2D array, representing a grid with X and Y space.
#[derive(Clone)]
pub struct Grid2D<T> {
    arr: T,
    width: usize,
    chunk_size: usize,
}

impl<T> Grid2D<T>
where
    T: Interpretation2D,
{
    pub fn new(arr: T, width: usize, chunk_size: usize) -> Grid2D<T> {
        debug_assert!(
            chunk_size > 0,
            "Grid2D chunk size must be greater than zero"
        );

        #[cfg(debug_assertions)]
        {
            let arr_size = arr.size();
            let inferred_height = arr_size / (width * chunk_size);
            let expected_size = inferred_height * width * chunk_size;

            assert!(
                expected_size == arr_size,
                "Grid2D::new() expected a collection of length {}, but got {} instead. Make sure width and chunk_size is correct.",
                expected_size,
                arr_size
            );
        }

        Grid2D {
            chunk_size,
            width,
            arr,
        }
    }

    pub fn write<B: IntoBounds2D<usize>, W: FnMut(usize, &mut [T::Item])>(
        &mut self,
        bounds: B,
        write: W,
    ) {
        let rows = row_ranges(bounds, self.width, self.chunk_size);
        self.arr.write(rows, write);
    }

    /// Copies the incoming slice of data onto the specified bounds within the grid.
    /// This assumes that the data has the same chunk_size as the grid.
    pub fn insert<B: IntoBounds2D<usize>>(&mut self, bounds: B, data: &[T::Item])
    where
        T::Item: Copy,
    {
        let bounds = bounds.to_bounds();
        let line_length = bounds.width() * self.chunk_size;

        // Get each line of the data
        let mut chunked = data.chunks_exact(line_length);

        self.write(bounds, |row, slice| match chunked.next() {
            None => panic!("Grid2D::insert() Expected incoming data to have {} rows but data stopped at row {}.", bounds.height(), row),
            Some(data) => {
                slice.copy_from_slice(data)
            },
        })
    }

    pub fn slice<B: IntoBounds2D<usize>>(&self, bounds: B) -> Vec<&T::Item> {
        let rows = row_ranges(bounds, self.width, self.chunk_size);
        let mapped: Vec<_> = rows.flat_map(|r| &self.arr[r]).collect();

        mapped
    }

    pub fn size(&self) -> Size2D<usize> {
        let width = self.width;
        let height = (self.arr.as_ref().len() / self.chunk_size) / self.width;

        Size2D::from((width, height))
    }

    pub fn index<P>(&self, position: P) -> usize
    where
        P: IntoPoint2D<usize>,
    {
        index_at(position, self.width, self.chunk_size)
    }

    pub fn values(&self) -> &[T::Item] {
        self.arr.values()
    }
}

impl<T> Display for Grid2D<T>
where
    T: Display + Interpretation2D,
    T::Item: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.size();
        let arr = self.arr.as_ref();

        let values: Vec<_> = arr
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

        write!(
            f,
            "Grid 2D ({w}x{h}; {}):\n{}",
            self.chunk_size,
            rows,
            w = size.width,
            h = size.height,
        )
    }
}

impl<T> Debug for Grid2D<T>
where
    T: Display + Interpretation2D,
    T::Item: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Grid2D<T> as Display>::fmt(&self, f)
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
        let mut ranges = super::row_ranges(bounds, 3, 1);

        assert_eq!(ranges.next(), Some(4..5));
        assert_eq!(ranges.next(), Some(7..8));

        // 0-1,   2-3,  4-5
        // 6-7,   X,    10-11,
        // 12-13, X,    16-17,
        let mut ranges = super::row_ranges(bounds, 3, 2);

        // A chunk size of 2 doubles the step of the range
        assert_eq!(ranges.next(), Some(8..10));
        assert_eq!(ranges.next(), Some(14..16));
    }

    #[test]
    #[should_panic]
    fn assert_incorrect_chunk_size() {
        Grid2D::new([0; { 8 * 3 }], 8, 2);
    }

    #[test]
    fn write() {
        let mut grid = Grid2D::new([0; { 3 * 3 * 2 }], 3, 2);

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

    #[test]
    fn insert() {
        let mut grid = Grid2D::new([0; { 3 * 3 * 2 }], 3, 2);

        let data_to_insert = [1, 2, 3, 4];
        grid.insert((1, 1, 1, 2), &data_to_insert);

        let expected_result = &[
            0, 0, 0, 0, 0, 0, //
            0, 0, 1, 2, 0, 0, //
            0, 0, 3, 4, 0, 0,
        ];

        assert_eq!(grid.values(), expected_result);
    }
}
