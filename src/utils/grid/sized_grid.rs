use crate::utils::coordinate::Position;
use crate::utils::grid::iterators::GridIter;
use crate::utils::grid::Grid;

/// A statically sized grid structure.
///
/// # Type Parameters
///
/// * `T` - The type of elements stored in the grid.
/// * `ROW` - The number of rows in the grid.
/// * `COL` - The number of columns in the grid.
pub struct SizedGrid<T, const ROW: usize, const COL: usize> {
    matrix: [[T; COL]; ROW],
}

impl<T, const ROW: usize, const COL: usize> SizedGrid<T, ROW, COL> {
    /// Creates an iterator over the grid.
    ///
    /// # Returns
    ///
    /// A `GridIter` over the grid.
    pub fn iter(&self) -> GridIter<'_, Self, T> {
        GridIter::new(self)
    }

    /// Creates a new `SizedGrid` from a 2D array.
    ///
    /// # Arguments
    ///
    /// * `grid` - A 2D array representing the grid.
    ///
    /// # Returns
    ///
    /// A new `SizedGrid` instance.
    #[allow(dead_code)]
    pub fn new(grid: [[T; COL]; ROW]) -> Self {
        Self { matrix: grid }
    }

    /// Returns the number of rows in the grid.
    ///
    /// # Returns
    ///
    /// The number of rows.
    #[inline(always)]
    pub fn num_rows(&self) -> usize {
        ROW
    }

    /// Returns the number of columns in the grid.
    ///
    /// # Returns
    ///
    /// The number of columns.
    #[inline(always)]
    pub fn num_cols(&self) -> usize {
        COL
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
    pub fn get(&self, position: Position) -> Option<&T> {
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
    pub fn get_mut(&mut self, position: Position) -> Option<&mut T> {
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
    pub fn is_valid_position(&self, position: Position) -> bool {
        position.i >= 0 && position.j >= 0 && position.i < ROW as i32 && position.j < COL as i32
    }
}

impl<T, const N: usize, const M: usize> Grid<T> for SizedGrid<T, N, M> {
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

    /// Returns a reference to the element at the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the element.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the element, or `None` if the position is invalid.
    fn get(&self, position: Position) -> Option<&T> {
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
    fn get_mut(&mut self, position: Position) -> Option<&mut T> {
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
    fn is_valid_position(&self, position: Position) -> bool {
        self.is_valid_position(position)
    }
}
