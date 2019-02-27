//! Game board logic.

/// Size of game board.
const SIZE: usize = 19;

#[derive(Copy, Clone, PartialEq)]
pub enum Stone {
    BLACK,
    WHITE,
	NOPE,
}
/// Stores game board information.
pub struct Gameboard {
	pub size: usize,
    pub cells: [[Stone; SIZE]; SIZE],
}

/// Creates a new game board.
impl Gameboard {
	pub fn new() -> Gameboard {
		Gameboard {
			size: SIZE,
			cells: [[Stone::NOPE; SIZE]; SIZE],
		}
	}
}