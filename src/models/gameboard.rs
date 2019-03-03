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
pub struct Gameboard {
	pub size: usize,
    pub cells: [[Stone; SIZE]; SIZE],
	pub selected_stone: Option<[usize; 2]>,
	pub preview_stone: Option<[usize; 2]>,
	change_window: bool,
}

/// Creates a new game board.
impl Gameboard {
	pub fn new() -> Gameboard {
		Gameboard {
			size: SIZE,
			cells: [[Stone::NOPE; SIZE]; SIZE],
			selected_stone: None,
			preview_stone: None,
			change_window: false,
		}
	}
}

impl GameViewModel for Gameboard {
    fn get_model(&mut self) -> &mut dyn Any {
        self
    }

    fn need_change_window(&self) -> bool {
        // self.change_window;
		false
	}
}