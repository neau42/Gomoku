//! Game board logic.
use crate::models::gameboard::*;
use crate::models::ia::*;
use crate::traits::view_model::*;
use std::any::Any;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Player {
    Human { nbr_capture: u8 },
    Ia { ia: IA, nbr_capture: u8 },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GameMode {
    PlayerVsPlayer,
    PlayerVsIa,
    IaVsPlayer,
    IaVsIa,
}

impl GameMode {
    pub fn get_index(&self) -> usize {
        match self {
            GameMode::PlayerVsPlayer => 0,
            GameMode::PlayerVsIa => 1,
            GameMode::IaVsPlayer => 2,
            GameMode::IaVsIa => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum GameResult {
    BlackWin,
    WhiteWin,
    Equality,
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
            Player::Human { nbr_capture } => *nbr_capture,
            Player::Ia { nbr_capture, .. } => *nbr_capture,
        }
    }
}

#[derive(Debug, Clone)]
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
    pub result: Option<GameResult>,
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
            result: None,
        }
    }

    pub fn new_with_game(mut game: Game, black_player: Player, white_player: Player, game_mode: &str) -> Game {
        game.black_player = black_player;
        game.white_player = white_player;
        game.game_mode = GameMode::new(game_mode);
        game.change_window = false;
        game
    }

    pub fn change_window(&mut self) {
        self.change_window = true;
    }

    pub fn update_last_move_time(&mut self) {
        let elapsed = self.timer.elapsed();
        let time =
            (elapsed.as_secs() as f64) + (f64::from(elapsed.subsec_nanos()) / 1_000_000_000.0);
        self.last_move_time = format!("Last move time: {}s", time);
        self.timer = Instant::now();
    }

    pub fn is_finish(&self) -> bool {
        self.result.is_some()
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
