use crate::utils::coordinate_system::Coordinate;
use crate::utils::grid::iterators::GridIter;

mod grid_slice;
pub mod sized_grid;
pub mod unsized_grid;

/// The `Grid` trait defines the interface for a grid structure.
/// It provides methods to get the number of rows and columns,
/// access rows and individual elements, and check if a coordinate is valid.
#[allow(dead_code)]
pub trait Grid<T> {
    /// Returns the number of rows in the grid.
    fn num_rows(&self) -> usize;

    /// Returns the number of columns in the grid.
    fn num_cols(&self) -> usize;

    /// Returns a reference to the row at the specified index.
    fn get_row(&self, row: usize) -> &[T];

    /// Returns a reference to the element at the specified coordinate, if valid.
    fn get(&self, coordinate: &Coordinate) -> Option<&T>;

    /// Checks if the specified coordinate is valid within the grid.
    fn is_valid_coordinate(&self, coordinate: &Coordinate) -> bool;

    /// Returns an iterator over the elements of the grid.
    ///
    /// # Type Parameters
    /// * `'a` - The lifetime of the references to the grid and its elements.
    ///
    /// # Returns
    /// A `GridIter` that iterates over the elements of the grid.
    fn iter<'a>(&'a self) -> GridIter<'a, Self, T>
    where
        T: 'a,
        Self: Sized;

    /// Returns the coordinate of the last element in the grid.
    ///
    /// # Returns
    /// A `Coordinate` representing the position of the last element in the grid.
    fn last_coordinate(&self) -> Coordinate {
        Coordinate::new((self.num_rows() - 1) as i32, (self.num_cols() - 1) as i32)
    }

    /// Applies a function to each element in the grid.
    ///
    /// # Type Parameters
    /// * `'a` - The lifetime of the references to the grid and its elements.
    /// * `F` - The type of the function to apply to each element.
    /// * `A` - The type of the result.
    ///
    /// # Arguments
    /// * `func` - The function to apply to each element. It takes a `Coordinate`, a reference to the element, and a mutable reference to the result.
    ///
    /// # Returns
    /// The result of applying the function to each element in the grid.
    fn foreach<F, A>(&self, func: F) -> A
    where
        F: Fn(Coordinate, &T, &mut A),
        A: Default,
        Self: Sized,
    {
        let mut accumulator: A = Default::default();

        for row in self.iter() {
            for (pos, e) in row {
                func(pos, e, &mut accumulator)
            }
        }

        accumulator
    }
}

#[allow(dead_code)]
pub trait GridMut<T>: Grid<T> {
    /// Returns a mutable reference to the row at the specified index.
    fn get_row_mut(&mut self, row: usize) -> &mut [T];

    /// Returns a mutable reference to the element at the specified coordinate, if valid.
    fn get_mut(&mut self, coordinate: &Coordinate) -> Option<&mut T>;
}

pub mod iterators {
    use crate::utils::coordinate_system::Coordinate;
    use crate::utils::grid::Grid;
    use std::marker::PhantomData;

    /// An iterator over the rows of a grid.
    pub struct GridIter<'a, G, T>
    where
        G: Grid<T>,
        T: 'a,
    {
        grid: &'a G,
        row: usize,
        _marker: PhantomData<&'a T>,
    }

    impl<'a, G, T> GridIter<'a, G, T>
    where
        G: Grid<T>,
    {
        /// Creates a new `GridIter` for the given grid.
        #[inline(always)]
        pub fn new(grid: &'a G) -> Self {
            Self {
                grid,
                row: 0,
                _marker: PhantomData,
            }
        }
    }

    impl<'a, G, T> Iterator for GridIter<'a, G, T>
    where
        G: Grid<T>,
    {
        type Item = RowIter<'a, T>;

        /// Advances the iterator and returns the next row iterator.
        fn next(&mut self) -> Option<Self::Item> {
            if self.row < self.grid.num_rows() {
                let row_iter = RowIter::new(self.grid.get_row(self.row), self.row, 0);
                self.row += 1;
                Some(row_iter)
            } else {
                None
            }
        }
    }

    /// An iterator over the elements of a row in a grid.
    ///
    /// # Type Parameters
    /// * `'a` - The lifetime of the references to the row elements.
    /// * `T` - The type of the elements in the row.
    pub struct RowIter<'a, T>
    where
        T: 'a,
    {
        /// A reference to the slice of row elements.
        row_item: &'a [T],
        /// The index of the current row.
        row: usize,
        /// The index of the current column.
        col: usize,
        /// A marker to indicate the lifetime of the row elements.
        _marker: PhantomData<&'a T>,
    }

    impl<'a, T> RowIter<'a, T> {
        pub fn new(row_item: &'a [T], row: usize, col: usize) -> Self {
            Self {
                row_item,
                row,
                col,
                _marker: PhantomData,
            }
        }
    }

    impl<'a, T> Iterator for RowIter<'a, T> {
        type Item = (Coordinate, &'a T);

        /// Advances the iterator and returns the next element in the row.
        fn next(&mut self) -> Option<Self::Item> {
            if self.col < self.row_item.len() {
                let coordinate = Coordinate::new(self.row as i32, self.col as i32);
                let value = &self.row_item[self.col];
                self.col += 1;
                Some((coordinate, value))
            } else {
                None
            }
        }
    }

    /// An iterator over the elements of a row in a grid.
    pub struct RowIterMut<'a, T>
    where
        T: 'a,
    {
        row_item: &'a mut [T],
        row: usize,
        col: usize,
    }

    impl<'a, T> RowIterMut<'a, T> {
        pub fn new(row_item: &'a mut [T], row: usize) -> Self {
            Self {
                row_item,
                row,
                col: 0,
            }
        }
    }

    impl<'a, T> Iterator for RowIterMut<'a, T> {
        type Item = (Coordinate, &'a mut T);

        /// Advances the iterator and returns the next element in the row.
        fn next(&mut self) -> Option<Self::Item> {
            let items = std::mem::take(&mut self.row_item);
            if let Some((item, rest)) = items.split_first_mut() {
                self.row_item = rest;
                let coordinate = Coordinate::new(self.row as i32, self.col as i32);
                self.col += 1;
                Some((coordinate, item))
            } else {
                None
            }
        }
    }
}
