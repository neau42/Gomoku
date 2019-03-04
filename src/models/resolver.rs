//! Game board logic.
use std::any::Any;
use crate::traits::view_model::*;


/// Size of game board.
const SIZE: usize = 19;

#[derive(Copy, Clone, PartialEq)]
pub enum Stone {
    BLACK,
    WHITE,
	NOPE,
}
/// Stores game board information.
pub struct Resolver {
	pub size: usize,
    pub cells: [[Stone; SIZE]; SIZE],
	pub test_switch: bool,

}

/// Creates a new game board.
impl Resolver {
	pub fn new() -> Resolver {
		Resolver {
			size: SIZE,
			cells: [[Stone::NOPE; SIZE]; SIZE],
			test_switch: true,
		}
	}

	pub fn set_stone_on_cell(&mut self, x: usize, y: usize, stone: Stone) -> bool {
		if self.cells[x][y] == Stone::NOPE {
			self.cells[x][y] = stone;
			true
		} else {
			false
		}
		
	}
}

impl GameViewModel for Resolver {
    fn get_model(&mut self) -> &mut dyn Any {
        self
    }

    fn need_change_window(&self) -> bool {
		false
	}
}