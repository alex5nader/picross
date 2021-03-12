/// A Picross board.
pub struct Board<C> {
    // Items are stored in row-major order.
    items: Vec<C>,
    width: usize,
    height: usize,
}

impl<C: Default + Clone> Board<C> {
    /// Creates a new board with `width` columns and `height` rows.
    pub fn of_size(width: usize, height: usize) -> Self {
        Self {
            items: vec![C::default(); width * height],
            width,
            height,
        }
    }
}

impl<C> Board<C> {
    #[cfg(test)]
    pub fn new_raw(items: Vec<C>, width: usize, height: usize) -> Self {
        Self {
            items,
            width,
            height,
        }
    }

    /// Returns a slice of the row at `index`.
    pub fn row(&self, index: usize) -> &[C] {
        let start = index * self.width;
        &self.items[start..(start + self.width)]
    }

    /// Returns an iterator over the column at `index`.
    pub fn column(&self, index: usize) -> impl Iterator<Item = &C> {
        Column {
            puzzle: self,
            index,
        }
    }

    /// Returns an iterator over the rows of this board.
    pub fn rows(&self) -> impl Iterator<Item = &[C]> {
        (0..self.height).map(move |row| self.row(row))
    }

    /// Returns an iterator over the columns of this board.
    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &C>> {
        (0..self.width).map(move |col| self.column(col))
    }

    /// Returns a reference to the item at `row` and `col`.
    pub fn get(&self, row: usize, col: usize) -> &C {
        &self.items[(row * self.width) + col]
    }

    /// Returns a mutable reference to the item at `row` and `col`.
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut C {
        &mut self.items[(row * self.width) + col]
    }
}

struct Column<'a, C> {
    puzzle: &'a Board<C>,
    index: usize,
}

impl<'a, C> Iterator for Column<'a, C> {
    type Item = &'a C;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.puzzle.items.get(self.index);
        self.index += self.puzzle.width;
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::Board;

    #[derive(PartialEq, Copy, Clone, Debug)]
    enum SimpleCell {
        Empty,
        Full,
    }
    impl Default for SimpleCell {
        fn default() -> Self {
            SimpleCell::Empty
        }
    }

    fn test_board() -> Board<SimpleCell> {
        use SimpleCell::*;

        Board {
            #[rustfmt::skip]
            items: vec![
                Empty, Full,  Full,  Empty, Empty,
                Full,  Empty, Empty, Full,  Empty,
                Empty, Full,  Full,  Empty, Empty,
            ],
            width: 5,
            height: 3,
        }
    }

    #[test]
    fn row_works() {
        use SimpleCell::*;

        let puzzle = test_board();

        let expected_rows = &[
            &[Empty, Full, Full, Empty, Empty],
            &[Full, Empty, Empty, Full, Empty],
            &[Empty, Full, Full, Empty, Empty],
        ];

        assert_eq!(puzzle.row(0), expected_rows[0]);
        assert_eq!(puzzle.row(1), expected_rows[1]);
        assert_eq!(puzzle.row(2), expected_rows[2]);

        for (expected_row, actual_row) in expected_rows.iter().zip(puzzle.rows()) {
            itertools::assert_equal(*expected_row, actual_row)
        }
    }

    #[test]
    fn col_works() {
        use SimpleCell::*;

        let puzzle = test_board();

        let expected_cols = &[
            &[Empty, Full, Empty],
            &[Full, Empty, Full],
            &[Full, Empty, Full],
            &[Empty, Full, Empty],
            &[Empty, Empty, Empty],
        ];

        itertools::assert_equal(puzzle.column(0), expected_cols[0]);
        itertools::assert_equal(puzzle.column(1), expected_cols[1]);
        itertools::assert_equal(puzzle.column(2), expected_cols[2]);
        itertools::assert_equal(puzzle.column(3), expected_cols[3]);
        itertools::assert_equal(puzzle.column(4), expected_cols[4]);

        for (expected_col, actual_col) in expected_cols.iter().zip(puzzle.columns()) {
            itertools::assert_equal(*expected_col, actual_col);
        }
    }

    #[test]
    fn get_works() {
        use SimpleCell::*;

        let puzzle = test_board();
        assert_eq!(*puzzle.get(0, 3), Empty);
        assert_eq!(*puzzle.get(2, 1), Full);
        assert_eq!(*puzzle.get(1, 4), Empty);
    }
}
