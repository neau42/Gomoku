use crate::traits::view_model::*;
use crate::traits::player::*;
use crate::models::game::Game;
use crate::models::ia::*;
use crate::models::human::*;
use crate::models::gameboard::*;
use std::any::Any;

pub struct GameBuilder {
    pub mode_index: usize,
    pub game_modes: [&'static str; 3],
    pub first_ia_depth: f32,
    pub second_ia_depth: f32,
    pub min_depth: f32,
    pub max_depth: f32,
    pub display_weight: bool,
    change_window: bool,
}

impl GameBuilder {
     pub fn new() -> GameBuilder {
        let min_depth = 1.0 as f32;
        let max_depth = 10.0 as f32;
        GameBuilder {
            mode_index: 0,
            game_modes: ["Player vs Player", "Player vs Ia", "Ia vs Ia"],
            first_ia_depth: min_depth,
            second_ia_depth: min_depth,
            min_depth,
            max_depth,
            display_weight: false,
            change_window: false,
        }
    }

    pub fn set_mode(&mut self, mode_index: usize) {
        self.mode_index = mode_index;
    }

    pub fn display_weight(&mut self, display_weight: bool) {
        self.display_weight = display_weight;
    }

    pub fn set_first_ia_depth(&mut self, depth: f32) {
        self.first_ia_depth = depth;
    }

    pub fn set_second_ia_depth(&mut self, depth: f32) {
        self.second_ia_depth = depth;
    }

    pub fn change_window(&mut self) {
        self.change_window = true;
    }

    pub fn build(&self) -> Game {
        let black_player: Box<Player> =  match self.mode_index {
            0 | 1 => Box::new(Human::new()),
            _ => Box::new(IA::new(self.first_ia_depth as u8)),
        };
        let white_player: Box<Player> =  match self.mode_index {
            0 => Box::new(Human::new()),
            _ => Box::new(IA::new(self.second_ia_depth as u8)),
        };
        Game {
            state: Gameboard::new(),
            black_player,
            white_player,
            is_black_turn: true,
        }
    }
}

impl GameViewModel for GameBuilder {
    fn get_model(&mut self) -> &mut dyn Any {
        self
    }

    fn need_change_window(&self) -> bool {
        self.change_window
    }
}