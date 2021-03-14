use crate::cell::CellValue;
use crate::{Board, Cell, ConstraintGroup, Puzzle};
use bitflags::bitflags;
use bitvec::prelude::*;

#[derive(Default)]
struct Status {
    row_status: BitVec,
    column_status: BitVec,
}

bitflags! {
    #[derive(Default)]
    struct Options: u8 {
        const AUTO_CROSS_COMPLETED = 0b0001;
    }
}

/// A picross game. Manages constraints, board, and completion.
pub struct Picross<C: CellValue> {
    puzzle: Puzzle<C>,
    board: Board<C>,
    status: Status,
    options: Options,
}

impl<C: CellValue> Picross<C> {
    /// Creates a new Picross game for `puzzle`.
    pub fn new(puzzle: Puzzle<C>) -> Self {
        let mut picross = Picross {
            status: Status {
                row_status: bitvec![0; puzzle.row_constraints().len()],
                column_status: bitvec![0; puzzle.column_constraints().len()],
            },
            options: Options::AUTO_CROSS_COMPLETED,
            board: Board::new_empty(puzzle.row_constraints().len(), puzzle.column_constraints().len()),
            puzzle,
        };
        for r in 0..picross.height() {
            picross.check_row(r);
        }
        for c in 0..picross.width() {
            picross.check_column(c);
        }
        picross
    }

    /// Returns an iterator over the cells in this game's board, and their positions.
    pub fn cells(&self) -> impl Iterator<Item = (usize, usize, &Cell<C>)> {
        self.board.cells()
    }

    /// Crosses out the cell at `row` and `column`.
    pub fn cross_out(&mut self, row: usize, column: usize) -> bool {
        *self.board.get_mut(row, column) = Cell::CrossedOut;
        self.check(row, column);
        self.is_solved()
    }

    /// Clears the cell at `row` and `column`.
    pub fn clear_at(&mut self, row: usize, column: usize) -> bool {
        *self.board.get_mut(row, column) = Cell::Empty;
        self.check(row, column);
        self.is_solved()
    }

    /// Places `value` into the cell at `row` and `column`.
    /// Returns whether or not the puzzle is solved afterwards.
    pub fn place_at(&mut self, value: C, row: usize, column: usize) -> bool {
        *self.board.get_mut(row, column) = Cell::Filled(value);
        self.check(row, column);
        self.is_solved()
    }

    fn check_row(&mut self, row: usize) {
        let completed = self.puzzle.row_is_solved(&self.board, row);
        self.status.row_status.set(row, completed);

        if self.options.contains(Options::AUTO_CROSS_COMPLETED) {
            if completed {
                for c in 0..self.width() {
                    let cell = self.board.get_mut(row, c);
                    if let Cell::Empty = *cell {
                        *cell = Cell::CrossedOut;
                    }
                }
            } else {
                for c in 0..self.width() {
                    if let Cell::CrossedOut = *self.board.get_mut(row, c) {
                        if !self.puzzle.column_is_solved(&self.board, c) {
                            *self.board.get_mut(row, c) = Cell::Empty
                        }
                    }
                }
            }
        }
    }

    fn check_column(&mut self, column: usize) {
        let completed = self.puzzle.column_is_solved(&self.board, column);
        self.status.column_status.set(column, completed);

        if self.options.contains(Options::AUTO_CROSS_COMPLETED) {
            if completed {
                for r in 0..self.height() {
                    let cell = self.board.get_mut(r, column);
                    if let Cell::Empty = *cell {
                        *cell = Cell::CrossedOut;
                    }
                }
            } else {
                for r in 0..self.height() {
                    if let Cell::CrossedOut = *self.board.get_mut(r, column) {
                        if !self.puzzle.row_is_solved(&self.board, r) {
                            *self.board.get_mut(r, column) = Cell::Empty;
                        }
                    }
                }
            }
        }
    }

    /// Checks whether or not the given row and column are solved and stores the result.
    fn check(&mut self, row: usize, column: usize) {
        self.check_row(row);
        self.check_column(column);
    }

    /// Returns the status of the puzzle's rows and columns.
    pub fn status(&self) -> (&BitVec, &BitVec) {
        (&self.status.row_status, &self.status.column_status)
    }

    /// Checks whether or not the puzzle is solved.  all the row and column constraints are satisfied.
    pub fn is_solved(&self) -> bool {
        self.status.row_status.all() && self.status.column_status.all()
    }

    /// Gets the cell at `row` and `column`.
    pub fn get(&self, row: usize, column: usize) -> &Cell<C> {
        self.board.get(row, column)
    }

    /// Returns the row constraint group.
    pub fn row_constraints(&self) -> &ConstraintGroup<C> {
        &self.puzzle.row_constraints()
    }

    /// Returns the column constraint group.
    pub fn column_constraints(&self) -> &ConstraintGroup<C> {
        &self.puzzle.column_constraints()
    }

    /// The width of this board.
    pub fn width(&self) -> usize {
        self.board.width()
    }

    /// The height of this board.
    pub fn height(&self) -> usize {
        self.board.height()
    }
}
