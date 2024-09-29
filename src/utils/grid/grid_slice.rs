use crate::utils::coordinate_system::Coordinate;
use crate::utils::grid::Grid;
use std::marker::PhantomData;
use std::ops::Range;

/// A view into a subset of a grid, defined by row and column ranges.
///
/// # Type Parameters
/// * `'grid` - The lifetime of the grid reference.
/// * `G` - The type of the grid, which must implement the `Grid` trait.
/// * `T` - The type of the elements in the grid, which must live at least as long as `'grid`.
#[derive(Debug)]
#[allow(dead_code)]
pub struct GridSlice<'grid, G, T>
where
    G: Grid<T>,
    T: 'grid,
{
    /// A reference to the grid.
    grid: &'grid G,
    /// The range of rows included in the view.
    row: Range<usize>,
    /// The range of columns included in the view.
    col: Range<usize>,
    /// Marker to indicate that GridView logically contains references to `T` with lifetime `'grid`.
    _marker: PhantomData<&'grid T>,
}

#[allow(dead_code)]
impl<'grid, G, T> GridSlice<'grid, G, T>
where
    G: Grid<T>,
{
    /// Creates a new `GridView` from the given grid and row/column ranges.
    ///
    /// # Arguments
    /// * `grid` - A reference to the grid.
    /// * `row` - The range of rows to include in the view.
    /// * `col` - The range of columns to include in the view.
    ///
    /// # Returns
    /// A new `GridView` instance.
    pub fn new(grid: &'grid G, row: Range<usize>, col: Range<usize>) -> Self {
        Self {
            grid,
            row,
            col,
            _marker: PhantomData,
        }
    }

    /// Creates a new `GridView` from an existing `GridView` and new row/column ranges.
    ///
    /// # Arguments
    /// * `grid` - A reference to an existing `GridView`.
    /// * `row` - The new range of rows to include in the view.
    /// * `col` - The new range of columns to include in the view.
    ///
    /// # Returns
    /// A new `GridView` instance.
    pub fn from_grid(grid: &GridSlice<'grid, G, T>, row: Range<usize>, col: Range<usize>) -> Self {
        Self {
            grid: grid.grid,
            row,
            col,
            _marker: PhantomData,
        }
    }

    /// Gets a reference to the element at the specified coordinate, if it is within the view.
    ///
    /// # Arguments
    /// * `coordinate` - The coordinate of the element to retrieve.
    ///
    /// # Returns
    /// An `Option` containing a reference to the element, or `None` if the coordinate is out of bounds.
    pub fn get(&self, coordinate: &Coordinate) -> Option<&T> {
        if !self.is_valid_coordinate(coordinate) {
            return None;
        }
        self.grid.get(coordinate)
    }

    /// Checks if the specified position is within the bounds of the view.
    ///
    /// # Arguments
    /// * `position` - The coordinate to check.
    ///
    /// # Returns
    /// `true` if the position is within the view, `false` otherwise.
    pub fn is_valid_coordinate(&self, position: &Coordinate) -> bool {
        position.i >= 0
            && position.j >= 0
            && self.row.contains(&(position.i as usize))
            && self.col.contains(&(position.j as usize))
    }

    /// Gets a slice of the specified row within the column range of the view.
    ///
    /// # Arguments
    /// * `row` - The index of the row to retrieve.
    ///
    /// # Returns
    /// A slice of the row within the column range of the view.
    pub fn row_as_slice(&self, row: usize) -> &[T] {
        &self.grid.get_row(row)[self.col.clone()]
    }

    /// Returns an iterator over the elements in the view.
    ///
    /// # Returns
    /// An iterator over the elements in the view.
    pub fn iter(&self) -> iterators::GridViewIter<'_, G, T> {
        iterators::GridViewIter::new(self)
    }
}

pub mod iterators {
    use crate::utils::grid::grid_slice::GridSlice;
    use crate::utils::grid::iterators::RowIter;
    use crate::utils::grid::Grid;
    use std::marker::PhantomData;

    /// An iterator over the elements in a `GridView`.
    pub struct GridViewIter<'grid, G, T>
    where
        G: Grid<T>,
    {
        grid_view: &'grid GridSlice<'grid, G, T>,
        row: usize,
        col: usize,
        _marker: PhantomData<&'grid T>,
    }

    impl<'grid, G, T> GridViewIter<'grid, G, T>
    where
        G: Grid<T>,
    {
        /// Creates a new `GridViewIter` for the given `GridView`.
        ///
        /// # Arguments
        /// * `grid_view` - A reference to the `GridView`.
        ///
        /// # Returns
        /// A new `GridViewIter` instance.
        pub fn new(grid_view: &'grid GridSlice<'grid, G, T>) -> Self {
            Self {
                grid_view,
                row: grid_view.row.start,
                col: grid_view.col.start,
                _marker: PhantomData,
            }
        }
    }

    impl<'grid, G, T> Iterator for GridViewIter<'grid, G, T>
    where
        G: Grid<T>,
        T: 'grid,
    {
        type Item = RowIter<'grid, T>;

        /// Advances the iterator and returns the next row iterator.
        ///
        /// # Returns
        /// An `Option` containing the next `RowIter`, or `None` if there are no more rows.
        fn next(&mut self) -> Option<Self::Item> {
            if self.grid_view.row.contains(&self.row) {
                let row_iter =
                    RowIter::new(self.grid_view.row_as_slice(self.row), self.row, self.col);
                self.row += 1;
                Some(row_iter)
            } else {
                None
            }
        }
    }
}
