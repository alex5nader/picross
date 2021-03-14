use crate::cell::CellValue;
use crate::{Board, Cell};
use itertools::Itertools;

/// Simple syntax for creating an entire constraint group.
#[macro_export]
macro_rules! constraints {
    ($([$($size:expr, $value:expr $(,)?);* $(;)?])*) => {
        vec![$(
            vec![$(
                $crate::ConstraintEntry { value: $value, size: $size }
            ),*]
        ),*]
    }
}

/// An entry in a constraint.
pub struct ConstraintEntry<C> {
    /// The cell this entry expects.
    pub value: C,
    /// The number of contiguous cells this entry expects.
    pub size: usize,
}

impl<C> From<(usize, C)> for ConstraintEntry<C> {
    fn from((size, value): (usize, C)) -> Self {
        ConstraintEntry { value, size }
    }
}

/// A constraint. Fully describes a row, column, etc.
pub type Constraint<C> = Vec<ConstraintEntry<C>>;
/// A group of constraints. Fully describes all rows, all columns, etc.
pub type ConstraintGroup<C> = Vec<Constraint<C>>;

/// A picross puzzle.
pub struct Puzzle<C> {
    row_constraints: ConstraintGroup<C>,
    column_constraints: ConstraintGroup<C>,
}

impl<C> Puzzle<C> {
    /// Creates a new puzzle with the given constraint groups.
    pub fn new(row_constraints: ConstraintGroup<C>, column_constraints: ConstraintGroup<C>) -> Self {
        Self {
            row_constraints,
            column_constraints,
        }
    }

    /// Returns the row constraint group.
    pub fn row_constraints(&self) -> &ConstraintGroup<C> {
        &self.row_constraints
    }

    /// Returns the column constraint group.
    pub fn column_constraints(&self) -> &ConstraintGroup<C> {
        &self.column_constraints
    }
}

impl<C: CellValue> Puzzle<C> {
    fn is_solved<'a, I: IntoIterator<Item = &'a Cell<C>>>(constraint: &'a Constraint<C>, cells: I) -> bool {
        let mut groups = cells.into_iter().peekable().batching(|it| {
            let value = loop {
                match it.next() {
                    None => return None, // out of cells
                    Some(Cell::Empty) | Some(Cell::CrossedOut) => continue,
                    Some(Cell::Filled(value)) => break value,
                }
            };
            let mut size = 1;

            loop {
                match it.peek() {
                    Some(Cell::Filled(next)) if *next == *value => {
                        it.next();
                        size += 1
                    }
                    _ => break,
                }
            }

            return Some((value, size));
        });

        let mut entries = constraint.iter().map(|c| (&c.value, c.size));

        loop {
            match (entries.next(), groups.next()) {
                (None, None) => break true,
                (Some(_), None) | (None, Some(_)) => break false,
                (Some(expected), Some(actual)) => {
                    if expected != actual {
                        break false;
                    }
                }
            }
        }
    }

    /// Checks whether the row in `board` at `index` is valid.
    pub fn row_is_solved(&self, board: &Board<C>, index: usize) -> bool {
        Self::is_solved(&self.row_constraints[index], board.row(index))
    }

    /// Checks whether the column in `board` at `index` is valid.
    pub fn column_is_solved(&self, board: &Board<C>, index: usize) -> bool {
        Self::is_solved(&self.column_constraints[index], board.column(index))
    }

    /// Checks whether the given board is a solution for this puzzle.
    /// Assumes the board has the same width and height as this puzzle.
    pub fn is_solved_by(&self, board: &Board<C>) -> bool {
        (0..self.row_constraints.len()).all(|i| self.row_is_solved(board, i))
            && {
                println!();
                true
            }
            && (0..self.column_constraints.len()).all(|i| self.column_is_solved(board, i))
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::ConstraintGroup;
    use crate::{Board, Puzzle};

    #[derive(PartialEq, Copy, Clone, Debug)]
    struct SimpleCell;

    fn test_board() -> Board<SimpleCell> {
        use crate::Cell::*;
        let filled = Filled(SimpleCell);

        #[rustfmt::skip]
        return Board::new_raw(vec![
            Empty,  filled, filled, Empty,  Empty,
            filled, Empty,  Empty,  filled, Empty,
            Empty,  filled, filled, Empty,  Empty,
        ], 5, 3);
    }

    fn test_puzzle() -> Puzzle<SimpleCell> {
        fn make(constraints: Vec<Vec<(usize, SimpleCell)>>) -> ConstraintGroup<SimpleCell> {
            return constraints
                .into_iter()
                .map(|constraint| constraint.into_iter().map(From::from).collect())
                .collect();
        }

        #[rustfmt::skip]
        return Puzzle::new(
            make(vec![
                vec![(2, SimpleCell)],
                vec![(1, SimpleCell), (1, SimpleCell)],
                vec![(2, SimpleCell)]
            ]),
            make(vec![
                vec![(1, SimpleCell)],
                vec![(1, SimpleCell), (1, SimpleCell)],
                vec![(1, SimpleCell), (1, SimpleCell)],
                vec![(1, SimpleCell)],
                vec![],
            ]),
        );
    }

    #[test]
    fn check_works() {
        let board = test_board();
        let puzzle = test_puzzle();

        assert!(puzzle.is_solved_by(&board));
    }

    #[test]
    fn check_fails() {
        #[rustfmt::skip]
        let board = {
            use crate::Cell::*;
            let filled = Filled(SimpleCell);

            Board::new_raw(vec![
                Empty,  filled, filled, Empty,  filled,
                filled, Empty,  Empty,  filled, Empty,
                Empty,  filled, filled, Empty,  filled,
            ], 5, 3)
        };
        let puzzle = test_puzzle();

        assert!(!puzzle.is_solved_by(&board));
    }
}
