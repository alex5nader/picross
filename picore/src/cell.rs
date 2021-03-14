/// Utility trait for types that can be cell values.
pub trait CellValue: std::fmt::Debug + PartialEq + Copy + Clone {}

impl<C: std::fmt::Debug + PartialEq + Copy + Clone> CellValue for C {}

/// A cell in a picross board.
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Cell<C: CellValue> {
    /// An empty cell.
    Empty,
    /// A crossed out cell.
    CrossedOut,
    /// A cell with a value.
    Filled(C),
}

impl<C: CellValue> Cell<C> {
    /// Whether or not this cell is ignored by constraints.
    pub fn is_ignored(&self) -> bool {
        match self {
            Cell::Empty | Cell::CrossedOut => true,
            Cell::Filled(_) => false,
        }
    }
}
