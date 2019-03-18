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

#[derive(Debug,PartialEq, Eq)]
pub enum GameMode {
	PlayerVsPlayer,
	PlayerVsIa,
	IaVsPlayer,
	IaVsIa,
}

impl GameMode {
	pub fn new(game_mode: &str) -> GameMode {
		match game_mode {
			"Player vs Player" => GameMode::PlayerVsPlayer,
			"Player vs Ia" => GameMode::PlayerVsIa,
			"Ia vs Player" => GameMode::IaVsPlayer,
			_ => GameMode::IaVsIa,
		}
	} 
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
	pub game_mode: GameMode,
	timer: Instant,
}

/// Creates a new game board.
impl Game {
	pub fn new(black_player: Player, white_player: Player, game_mode: &str) -> Game {
		let start_state = Gameboard::new();
        Game {
            state: start_state.clone(),
            black_player,
            white_player,
			last_move_time: "Last move time: 0.0s".to_string(),
			all_state: vec![start_state],
            current_stone: Stone::BLACK,
			change_window: false,
			game_mode: GameMode::new(game_mode),
			timer: Instant::now(),
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