use crate::models::game::Player;
use crate::models::game::*;
use crate::models::ia::*;
use crate::traits::view_model::*;
use std::any::Any;

pub struct GameBuilder {
    pub mode_index: usize,
    pub game_modes: [&'static str; 4],
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
            game_modes: [
                "Player vs Player",
                "Player vs Ia",
                "Ia vs Player",
                "Ia vs Ia",
            ],
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
        let ia_new = |depth: f32| -> Player {
            Player::Ia {
                ia: IA::new(depth as u8),
                nbr_capture: 0 as u8,
            }
        };

        let human_new = || -> Player {
            Player::Human {
                nbr_capture: 0 as u8,
            }
        };

        let black_player: Player = match self.mode_index {
            0 | 1 => human_new(),
            _ => ia_new(self.first_ia_depth),
        };
        let white_player: Player = match self.mode_index {
            0 | 2 => human_new(),
            1 => ia_new(self.first_ia_depth),
            _ => ia_new(self.second_ia_depth),
        };
        Game::new(black_player, white_player, self.game_modes[self.mode_index])
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
