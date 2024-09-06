use crate::utils::coordinate::Position;
use crate::utils::gird::iterators::GridIter;
use std::fmt::Debug;

#[repr(transparent)]
pub struct Grid<T> {
    matrix: Box<[Box<[T]>]>,
}
impl<T> Grid<T> {
    pub fn iter(&self) -> GridIter<T> {
        GridIter::new(self)
    }

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
    
    #[allow(dead_code)]
    pub fn from_box(grid: Box<[Box<[T]>]>) -> Self {
        Self { matrix: grid }
    }

    #[inline(always)]
    pub fn num_rows(&self) -> usize {
        self.matrix.len()
    }

    #[inline(always)]
    pub fn num_cols(&self) -> usize {
        self.matrix[0].len()
    }

    #[inline(always)]
    pub fn get(&self, position: Position) -> Option<&T> {
        if self.is_valid_position(position) {
            Some(&self.matrix[position.i as usize][position.j as usize])
        } else {
            None
        }
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn get_mut(&mut self, position: Position) -> Option<&mut T> {
        if self.is_valid_position(position) {
            Some(&mut self.matrix[position.i as usize][position.j as usize])
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn is_valid_position(&self, position: Position) -> bool {
        position.i >= 0
            && position.j >= 0
            && position.i < self.num_rows() as i32
            && position.j < self.num_cols() as i32
    }
}
pub mod iterators {
    use crate::utils::coordinate::Position;
    use crate::utils::gird::Grid;

    pub struct GridIter<'a, T> {
        grid: &'a Grid<T>,
        /// ===
        row: usize,
        /// | | |
        col: usize,
    }

    impl<'a, T> GridIter<'a, T> {
        #[inline(always)]
        pub fn new(grid: &'a Grid<T>) -> Self {
            Self {
                grid,
                row: 0,
                col: 0,
            }
        }
    }

    impl<'a, T> Iterator for GridIter<'a, T> {
        type Item = RowIter<'a, T>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.row < self.grid.num_rows() {
                let row_iter = RowIter {
                    row_item: self.grid.matrix[self.row].as_ref(),
                    row: self.row,
                    col: self.col,
                };
                self.row += 1;
                Some(row_iter)
            } else {
                None
            }
        }
    }

    pub struct RowIter<'a, T> {
        row_item: &'a [T],
        row: usize,
        col: usize,
    }

    impl<'a, T> Iterator for RowIter<'a, T> {
        type Item = (Position, &'a T);

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
}

impl<T: Debug> Debug for Grid<T> {
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
