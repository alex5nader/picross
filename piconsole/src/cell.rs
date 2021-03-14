//! Various types of cells in picross.

use picore::Cell;

/// Binary cell. Either full or empty.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SimpleCell;

impl SimpleCell {
    /// Gets the character representation of the given simple cell.
    pub fn char_repr(cell: &Cell<SimpleCell>) -> char {
        match *cell {
            Cell::Empty => '.',
            Cell::CrossedOut => '/',
            Cell::Filled(_) => '#',
        }
    }
}

// /// Cell containing characters.
// #[derive(Copy, Clone)]
// pub enum Char {
//     /// Empty cell.
//     Empty,
//     /// Cell with a character.
//     Value(char),
// }
//
// impl Default for Char {
//     fn default() -> Self {
//         Char::Empty
//     }
// }
