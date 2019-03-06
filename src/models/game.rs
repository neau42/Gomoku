//! Game board logic.
use crate::traits::view_model::*;
use crate::models::gameboard::*;
use std::any::Any;


pub struct Game {
	pub state: Gameboard,
	// pub players: [Box<Player>; 2],
	// pub is_black_turn: bool,
}

/// Creates a new game board.
impl Game {
	pub fn new() -> Game {
		Game {
			state: Gameboard::new(),
		}
	}
}

impl GameViewModel for Game {
    fn get_model(&mut self) -> &mut dyn Any {
        self
    }

    fn need_change_window(&self) -> bool {
		false
	}
}