use crate::utils::coordinate_system::Coordinate;
use crate::utils::grid::iterators::{GridIter, RowIterMut};
use crate::utils::grid::Grid;
use std::fmt::Debug;
use std::iter::Enumerate;
use std::marker::PhantomData;
use std::slice::IterMut;

/// A dynamically sized grid structure.
///
/// # Type Parameters
///
/// * `T` - The type of elements stored in the grid.
#[repr(transparent)]
pub struct UnsizedGrid<T> {
    matrix: Box<[Box<[T]>]>,
}

impl<T> UnsizedGrid<T> {
    /// Creates an iterator over the grid.
    ///
    /// # Returns
    ///
    /// A `GridIter` over the grid.
    pub fn iter(&self) -> GridIter<'_, Self, T> {
        GridIter::new(self)
    }

    /// Creates a new `UnsizedGrid` from a 2D vector.
    ///
    /// # Arguments
    ///
    /// * `grid` - A 2D vector representing the grid.
    ///
    /// # Returns
    ///
    /// A new `UnsizedGrid` instance.
    #[allow(dead_code)]
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        let grid: Box<[Box<[T]>]> = grid
            .into_iter()
            .map(|row| row.into_boxed_slice())
            .collect::<Vec<Box<[T]>>>()
            .into_boxed_slice();

        assert!(grid.len() > 0);
        assert!(grid[0].len() > 0);

        Self { matrix: grid }
    }

    /// Creates a new `UnsizedGrid` from a boxed 2D slice.
    ///
    /// # Arguments
    ///
    /// * `grid` - A boxed 2D slice representing the grid.
    ///
    /// # Returns
    ///
    /// A new `UnsizedGrid` instance.
    #[allow(dead_code)]
    pub fn from_box(grid: Box<[Box<[T]>]>) -> Self {
        Self { matrix: grid }
    }

    /// Returns the number of rows in the grid.
    ///
    /// # Returns
    ///
    /// The number of rows.
    #[inline(always)]
    pub fn num_rows(&self) -> usize {
        self.matrix.len()
    }

    /// Returns the number of columns in the grid.
    ///
    /// # Returns
    ///
    /// The number of columns.
    #[inline(always)]
    pub fn num_cols(&self) -> usize {
        self.matrix[0].len()
    }

    /// Returns a reference to the element at the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the element.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the element, or `None` if the position is invalid.
    #[inline(always)]
    pub fn get(&self, position: Coordinate) -> Option<&T> {
        if self.is_valid_position(position) {
            Some(&self.matrix[position.i as usize][position.j as usize])
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the element.
    ///
    /// # Returns
    ///
    /// An `Option` containing a mutable reference to the element, or `None` if the position is invalid.
    #[allow(dead_code)]
    #[inline(always)]
    pub fn get_mut(&mut self, position: Coordinate) -> Option<&mut T> {
        if self.is_valid_position(position) {
            Some(&mut self.matrix[position.i as usize][position.j as usize])
        } else {
            None
        }
    }

    /// Checks if the specified position is valid within the grid.
    ///
    /// # Arguments
    ///
    /// * `position` - The position to check.
    ///
    /// # Returns
    ///
    /// `true` if the position is valid, `false` otherwise.
    #[inline(always)]
    pub fn is_valid_position(&self, position: Coordinate) -> bool {
        position.i >= 0
            && position.j >= 0
            && position.i < self.num_rows() as i32
            && position.j < self.num_cols() as i32
    }
}

impl<T: Debug> Debug for UnsizedGrid<T> {
    /// Formats the grid using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.matrix.iter() {
            for cell in row.iter() {
                write!(f, "{:?} ", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Grid<T> for UnsizedGrid<T> {
    /// Returns the number of rows in the grid.
    fn num_rows(&self) -> usize {
        self.num_rows()
    }

    /// Returns the number of columns in the grid.
    fn num_cols(&self) -> usize {
        self.num_cols()
    }

    /// Returns a reference to the row at the specified index.
    ///
    /// # Arguments
    ///
    /// * `row` - The index of the row.
    ///
    /// # Returns
    ///
    /// A reference to the row.
    fn get_row(&self, row: usize) -> &[T] {
        &self.matrix[row]
    }

    /// Returns a mutable reference to the row at the specified index.
    ///
    /// # Arguments
    ///
    /// * `row` - The index of the row.
    ///
    /// # Returns
    ///
    /// A reference to the row.
    fn get_row_mut(&mut self, row: usize) -> &mut [T] {
        &mut self.matrix[row]
    }

    /// Returns a reference to the element at the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the element.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the element, or `None` if the position is invalid.
    fn get(&self, position: Coordinate) -> Option<&T> {
        self.get(position)
    }

    /// Returns a mutable reference to the element at the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the element.
    ///
    /// # Returns
    ///
    /// An `Option` containing a mutable reference to the element, or `None` if the position is invalid.
    fn get_mut(&mut self, position: Coordinate) -> Option<&mut T> {
        self.get_mut(position)
    }

    /// Checks if the specified position is valid within the grid.
    ///
    /// # Arguments
    ///
    /// * `position` - The position to check.
    ///
    /// # Returns
    ///
    /// `true` if the position is valid, `false` otherwise.
    fn is_valid_position(&self, position: Coordinate) -> bool {
        self.is_valid_position(position)
    }
}

pub struct GridIterMut<'a, T>
where
    T: 'a,
{
    grid_rows: Enumerate<IterMut<'a, Box<[T]>>>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> GridIterMut<'a, T>
where
    T: 'a,
{
    #[allow(dead_code)]
    pub fn new(grid: &'a mut UnsizedGrid<T>) -> Self {
        let enumerated_rows: Enumerate<IterMut<Box<[T]>>> = grid.matrix.iter_mut().enumerate();
        Self {
            grid_rows: enumerated_rows,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for GridIterMut<'a, T>
where
    T: 'a,
{
    type Item = RowIterMut<'a, T>;

    /// Advances the iterator and returns the next row iterator.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((row, row_item)) = self.grid_rows.next() {
            let row_iter = RowIterMut::new(row_item.as_mut(), row);
            Some(row_iter)
        } else {
            None
        }
    }
}
