use crate::cell::CellValue;
use crate::Cell;
use itertools::Itertools;

/// A Picross board.
pub struct Board<C: CellValue> {
    // Items are stored in row-major order.
    items: Vec<Cell<C>>,
    width: usize,
    height: usize,
}

impl<C: CellValue> Board<C> {
    /// Creates a new board with `width` columns and `height` rows.
    pub fn new_empty(width: usize, height: usize) -> Self {
        Self {
            items: vec![Cell::Empty; width * height],
            width,
            height,
        }
    }
}

impl<C: CellValue> Board<C> {
    #[cfg(test)]
    pub fn new_raw(items: Vec<Cell<C>>, width: usize, height: usize) -> Self {
        Self { items, width, height }
    }

    /// The width of this board.
    pub fn width(&self) -> usize {
        self.width
    }

    /// The height of this board.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns a slice of the row at `index`.
    pub fn row(&self, index: usize) -> &[Cell<C>] {
        let start = index * self.width;
        &self.items[start..(start + self.width)]
    }

    /// Returns an iterator over the column at `index`.
    pub fn column(&self, index: usize) -> impl Iterator<Item = &Cell<C>> {
        Column { puzzle: self, index }
    }

    /// Returns an iterator over the rows of this board.
    pub fn rows(&self) -> impl Iterator<Item = &[Cell<C>]> {
        (0..self.height).map(move |row| self.row(row))
    }

    /// Returns an iterator over the columns of this board.
    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &Cell<C>>> {
        (0..self.width).map(move |col| self.column(col))
    }

    /// Returns an iterator over the cells in this board, and their positions.
    pub fn cells(&self) -> impl Iterator<Item = (usize, usize, &Cell<C>)> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(move |(r, c)| (r, c, self.get(r, c)))
    }

    /// Returns a reference to the item at `row` and `col`.
    pub fn get(&self, row: usize, col: usize) -> &Cell<C> {
        &self.items[(row * self.width) + col]
    }

    /// Returns a mutable reference to the item at `row` and `col`.
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut Cell<C> {
        &mut self.items[(row * self.width) + col]
    }
}

struct Column<'a, C: CellValue> {
    puzzle: &'a Board<C>,
    index: usize,
}

impl<'a, C: CellValue> Iterator for Column<'a, C> {
    type Item = &'a Cell<C>;

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
    struct SimpleCell;

    fn test_board() -> Board<SimpleCell> {
        use crate::Cell::*;
        let filled = Filled(SimpleCell);

        Board {
            #[rustfmt::skip]
            items: vec![
                Empty,  filled, filled, Empty,  Empty,
                filled, Empty,  Empty,  filled, Empty,
                Empty,  filled, filled, Empty,  Empty,
            ],
            width: 5,
            height: 3,
        }
    }

    #[test]
    fn row_works() {
        use crate::Cell::*;
        let filled = Filled(SimpleCell);

        let puzzle = test_board();

        #[rustfmt::skip]
        let expected_rows = &[
            &[Empty,  filled, filled, Empty,  Empty],
            &[filled, Empty,  Empty,  filled, Empty],
            &[Empty,  filled, filled, Empty,  Empty],
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
        use crate::Cell::*;
        let filled = Filled(SimpleCell);

        let puzzle = test_board();

        #[rustfmt::skip]
        let expected_cols = &[
            &[Empty,  filled, Empty],
            &[filled, Empty,  filled],
            &[filled, Empty,  filled],
            &[Empty,  filled, Empty],
            &[Empty,  Empty,  Empty],
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
        use crate::Cell::*;
        let filled = Filled(SimpleCell);

        let puzzle = test_board();
        assert_eq!(*puzzle.get(0, 3), Empty);
        assert_eq!(*puzzle.get(2, 1), filled);
        assert_eq!(*puzzle.get(1, 4), Empty);
    }
}
