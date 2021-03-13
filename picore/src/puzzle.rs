use crate::Board;

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

impl<C: Default + PartialEq> Puzzle<C> {
    fn is_solved<'a, I: IntoIterator<Item = &'a C>>(constraint: &'a Constraint<C>, items: I) -> bool {
        let mut items = items.into_iter();
        for entry in constraint {
            for _ in 0..entry.size {
                let mut invalid = true;
                while let Some(next) = items.next() {
                    if *next == Default::default() {
                        continue;
                    }
                    invalid = *next != entry.value;
                    break;
                }
                if invalid {
                    return false;
                }
            }
        }
        true
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
            && (0..self.column_constraints.len()).all(|i| self.column_is_solved(board, i))
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::tests::SimpleCell::Empty;
    use crate::puzzle::ConstraintGroup;
    use crate::{Board, Puzzle};

    #[derive(PartialEq, Copy, Clone, Debug)]
    enum SimpleCell {
        Empty,
        Full,
    }
    impl Default for SimpleCell {
        fn default() -> Self {
            Empty
        }
    }

    fn test_board() -> Board<SimpleCell> {
        use SimpleCell::*;

        #[rustfmt::skip]
        return Board::new_raw(vec![
            Empty, Full,  Full,  Empty, Empty,
            Full,  Empty, Empty, Full,  Empty,
            Empty, Full,  Full,  Empty, Empty,
        ], 5, 3);
    }

    fn test_puzzle() -> Puzzle<SimpleCell> {
        use SimpleCell::*;

        fn make(constraints: Vec<Vec<(usize, SimpleCell)>>) -> ConstraintGroup<SimpleCell> {
            return constraints
                .into_iter()
                .map(|constraint| constraint.into_iter().map(From::from).collect())
                .collect();
        }

        #[rustfmt::skip]
        return Puzzle::new(
            make(vec![
                vec![(2, Full)],
                vec![(1, Full), (1, Full)],
                vec![(2, Full)]
            ]),
            make(vec![
                vec![(1, Full)],
                vec![(1, Full), (1, Full)],
                vec![(1, Full), (1, Full)],
                vec![(1, Full)],
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
}
