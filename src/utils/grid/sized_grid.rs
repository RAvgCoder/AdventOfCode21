use crate::utils::coordinate_system::Coordinate;
use crate::utils::grid::iterators::{GridIter, RowIterMut};
use crate::utils::grid::{Grid, GridMut};
use std::fmt::{Debug, Formatter};
use std::iter::Enumerate;
use std::marker::PhantomData;
use std::slice::IterMut;

/// A statically sized grid structure.
///
/// # Type Parameters
///
/// * `T` - The type of elements stored in the grid.
/// * `ROW` - The number of rows in the grid.
/// * `COL` - The number of columns in the grid.
pub struct SizedGrid<T, const ROW: usize, const COL: usize> {
    pub matrix: [[T; COL]; ROW],
}

impl<T, const ROW: usize, const COL: usize> SizedGrid<T, ROW, COL> {
    /// Creates an iterator over the grid.
    ///
    /// # Returns
    ///
    /// A `GridIter` over the grid.
    #[allow(dead_code)]
    pub fn iter(&self) -> GridIter<'_, Self, T> {
        GridIter::new(self)
    }

    /// Creates a mutable iterator over the grid.
    ///
    /// # Returns
    ///
    /// A `GridIter` over the grid.
    pub fn iter_mut(&mut self) -> GridIterMut<'_, T, ROW, COL> {
        GridIterMut::new(self)
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
    #[inline(always)]
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
    pub fn get(&self, position: &Coordinate) -> Option<&T> {
        if self.is_valid_coordinate(position) {
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
    pub fn get_mut(&mut self, position: &Coordinate) -> Option<&mut T> {
        if self.is_valid_coordinate(&position) {
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
    pub fn is_valid_coordinate(&self, position: &Coordinate) -> bool {
        position.i >= 0 && position.j >= 0 && position.i < ROW as i32 && position.j < COL as i32
    }
}

impl<T: Debug, const ROW: usize, const COL: usize> Debug for SizedGrid<T, ROW, COL> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SizedGrid: (ROW: {} x COL:{}) {{", ROW, COL)?;
        for rows in &self.matrix {
            write!(f, "\t")?;
            for cell in rows.iter() {
                write!(f, "{:?} ", cell)?;
            }
            writeln!(f)?;
        }
        write!(f, "}}")?;
        Ok(())
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
    fn get(&self, position: &Coordinate) -> Option<&T> {
        self.get(position)
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
    fn is_valid_coordinate(&self, position: &Coordinate) -> bool {
        self.is_valid_coordinate(position)
    }

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
    {
        GridIter::new(self)
    }
}

impl<T, const N: usize, const M: usize> GridMut<T> for SizedGrid<T, N, M> {
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

    /// Returns a mutable reference to the element at the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the element.
    ///
    /// # Returns
    ///
    /// An `Option` containing a mutable reference to the element, or `None` if the position is invalid.
    fn get_mut(&mut self, position: &Coordinate) -> Option<&mut T> {
        self.get_mut(position)
    }
}

pub struct GridIterMut<'a, T, const ROW: usize, const COL: usize>
where
    T: 'a,
{
    grid_rows: Enumerate<IterMut<'a, [T; COL]>>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T, const ROW: usize, const COL: usize> GridIterMut<'a, T, ROW, COL>
where
    T: 'a,
{
    pub fn new(grid: &'a mut SizedGrid<T, ROW, COL>) -> Self {
        let enumerated_rows = grid.matrix.iter_mut().enumerate();
        Self {
            grid_rows: enumerated_rows,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, const ROW: usize, const COL: usize> Iterator for GridIterMut<'a, T, ROW, COL>
where
    T: 'a,
{
    type Item = RowIterMut<'a, T>;

    /// Advances the iterator and returns the next row iterator.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((row, row_item)) = self.grid_rows.next() {
            let row_iter = RowIterMut::new(row_item.as_mut_slice(), row);
            Some(row_iter)
        } else {
            None
        }
    }
}
