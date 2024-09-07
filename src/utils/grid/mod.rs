use crate::utils::coordinate::Position;
use std::fmt::Debug;
pub mod sized_grid;
pub mod unsized_grid;

/// The `Grid` trait defines the interface for a grid structure.
/// It provides methods to get the number of rows and columns,
/// access rows and individual elements, and check if a position is valid.
pub trait Grid<T> {
    /// Returns the number of rows in the grid.
    fn num_rows(&self) -> usize;

    /// Returns the number of columns in the grid.
    fn num_cols(&self) -> usize;

    /// Returns a reference to the row at the specified index.
    fn get_row(&self, row: usize) -> &[T];

    /// Returns a mut reference to the row at the specified index.
    fn get_row_mut(&mut self, row: usize) -> &mut [T];

    /// Returns a reference to the element at the specified position, if valid.
    fn get(&self, position: Position) -> Option<&T>;

    /// Returns a mutable reference to the element at the specified position, if valid.
    fn get_mut(&mut self, position: Position) -> Option<&mut T>;

    /// Checks if the specified position is valid within the grid.
    fn is_valid_position(&self, position: Position) -> bool;
}

/// Implements the `Debug` trait for any type that implements the `Grid` trait.
/// This allows for formatted output of the grid's contents.
impl<T: Debug> Debug for dyn Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for num_col in 0..self.num_cols() {
            for cell in self.get_row(num_col).iter() {
                write!(f, "{:?} ", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub mod iterators {
    use crate::utils::coordinate::Position;
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
                let row_iter = RowIter {
                    row_item: self.grid.get_row(self.row),
                    row: self.row,
                    col: 0,
                };
                self.row += 1;
                Some(row_iter)
            } else {
                None
            }
        }
    }

    /// An iterator over the elements of a row in a grid.
    pub struct RowIter<'a, T> {
        row_item: &'a [T],
        row: usize,
        col: usize,
    }

    impl<'a, T> Iterator for RowIter<'a, T> {
        type Item = (Position, &'a T);

        /// Advances the iterator and returns the next element in the row.
        fn next(&mut self) -> Option<Self::Item> {
            if self.col < self.row_item.len() {
                let position = Position::new(self.row as i32, self.col as i32);
                let value = &self.row_item[self.col];
                self.col += 1;
                Some((position, value))
            } else {
                None
            }
        }
    }

    pub struct GridIterMut<'a, G, T>
    where
        G: Grid<T>,
        T: 'a,
    {
        grid: &'a mut G,
        row: usize,
        _marker: PhantomData<&'a mut T>,
    }

    impl<'a, G, T> GridIterMut<'a, G, T>
    where
        G: Grid<T>,
        T: 'a,
    {
        pub fn new(grid: &'a mut G) -> Self {
            Self {
                grid,
                row: 0,
                _marker: PhantomData,
            }
        }
    }

    impl<'a, G, T> Iterator for GridIterMut<'a, G, T>
    where
        G: Grid<T>,
        T: 'a,
    {
        type Item = RowIterMut<'a, T>;

        /// Advances the iterator and returns the next row iterator.
        fn next(&mut self) -> Option<Self::Item> {
            if self.row < self.grid.num_rows() {
                let row_item = self.grid.get_row_mut(self.row);
                let row_iter = RowIterMut {
                    row_item,
                    row: self.row,
                    col: 0,
                };
                self.row += 1;
                Some(row_iter)
            } else {
                None
            }
            
            // if self.row < self.grid.num_rows() {
            //     let row_item = self.grid.get_row_mut(self.row);
            //     let row_iter = RowIterMut {
            //         row_item,
            //         row: self.row,
            //         col: 0,
            //     };
            //     self.row += 1;
            //     Some(row_iter)
            // } else {
            //     None
            // }
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

    impl<'a, T> Iterator for RowIterMut<'a, T> {
        type Item = (Position, &'a mut T);

        /// Advances the iterator and returns the next element in the row.
        fn next(&mut self) -> Option<Self::Item> {
            let items = std::mem::replace(&mut self.row_item, &mut []);
            if let Some((item, rest)) = items.split_first_mut() {
                self.row_item = rest;
                let position = Position::new(self.row as i32, self.col as i32);
                self.col += 1;
                Some((position, item))
            }else {
                None
            }

            // if self.col < self.row_item.len() {
            //     let position = Position::new(self.row as i32, self.col as i32);
            //     let value = &mut self.row_item[self.col];
            //     self.col += 1;
            //     Some((position, value))
            // } else {
            //     None
            // }
        }
    }
}
