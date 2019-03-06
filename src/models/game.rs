//! Game board logic.
use crate::traits::view_model::*;
use crate::traits::player::*;
use crate::models::gameboard::*;
use std::any::Any;


pub struct Game {
	pub state: Gameboard,
	pub black_player: Box<Player>,
	pub white_player: Box<Player>,
	pub is_black_turn: bool,
}

/// Creates a new game board.
impl Game {
}

impl GameViewModel for Game {
    fn get_model(&mut self) -> &mut dyn Any {
        self
    }

    fn need_change_window(&self) -> bool {
		false
	}
}