//! Game board logic.
use crate::traits::view_model::*;
use crate::models::gameboard::*;
use crate::models::ia::*;
use std::any::Any;

pub enum Player {
    Human{nbr_capture: u8},
    Ia{ia: IA, nbr_capture: u8},
}

pub struct Game {
	pub state: Gameboard,
	pub black_player: Player,
	pub white_player: Player,
	pub all_state: Vec<Gameboard>,
    pub current_stone: Stone,
}

/// Creates a new game board.
impl Game {
	pub fn new(black_player: Player, white_player: Player) -> Game {
		let start_state = Gameboard::new();
        Game {
            state: start_state.clone(),
            black_player,
            white_player,
			all_state: vec![start_state],
            current_stone: Stone::BLACK,
		}
	}

	pub fn get_current_player(&mut self) -> &mut Player {
		match self.current_stone {
			Stone::WHITE => &mut self.white_player,
			_ => &mut self.black_player,
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