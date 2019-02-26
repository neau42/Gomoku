//! Game board logic.

/// Size of game board.
const SIZE: usize = 19;

#[derive(Copy, Clone)]
pub enum Stone {
    BLACK,
    WHITE,
	NOPE,
}
/// Stores game board information.
pub struct Gameboard {
    /// Stores the content of the cells.
    /// `0` is an empty cell.
    pub cells: [[Stone; SIZE]; SIZE],
}

impl Gameboard {
    /// Creates a new game board.
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[Stone::NOPE; SIZE]; SIZE],
        }
    }
}