//! Game board logic.
use crate::traits::view_model::*;
use crate::models::gameboard::*;
use crate::models::ia::*;
use std::time::Duration;
use std::time::Instant;
use std::any::Any;

pub enum Player {
    Human{nbr_capture: u8},
    Ia{ia: IA, nbr_capture: u8},
}

impl Player {
	pub fn captures(&self) -> u8 {
		match self {
			Player::Human{nbr_capture} => *nbr_capture,
			Player::Ia{nbr_capture, ..} => *nbr_capture
		}
	}
}

pub struct Game {
	pub state: Gameboard,
	pub black_player: Player,
	pub white_player: Player,
	pub last_move_time: String,
	pub all_state: Vec<Gameboard>,
    pub current_stone: Stone,
	pub change_window: bool,
	timer: Instant,
}

/// Creates a new game board.
impl Game {
	pub fn new(black_player: Player, white_player: Player) -> Game {
		let start_state = Gameboard::new();
        Game {
            state: start_state.clone(),
            black_player,
            white_player,
			last_move_time: "Last move time: 0.0s".to_string(),
			all_state: vec![start_state],
            current_stone: Stone::BLACK,
			change_window: false,
			timer: Instant::now()
		}
	}

	pub fn get_current_player(&mut self) -> &mut Player {
		match self.current_stone {
			Stone::WHITE => &mut self.white_player,
			_ => &mut self.black_player,
		}
	}

	pub fn change_window(&mut self) {
		self.change_window = true;
	}

	pub fn update_last_move_time(&mut self) {
		let elapsed = self.timer.elapsed();
		let time = ((elapsed.as_secs() as f64) + (f64::from(elapsed.subsec_nanos()) / 1_000_000_000.0));
		self.last_move_time = format!("Last move time: {}s", time);
		self.timer = Instant::now();
	}
}

impl GameViewModel for Game {
    fn get_model(&mut self) -> &mut dyn Any {
        self
    }

    fn need_change_window(&self) -> bool {
		self.change_window
	}
}