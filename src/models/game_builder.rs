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
    pub game: Option<Game>,
}

impl GameBuilder {
    pub fn new(game: Option<Game>) -> GameBuilder {
        let min_depth = 1.0 as f32;
        let max_depth = 10.0 as f32;
        let (mode_index, first_ia_depth, second_ia_depth) = if game.is_none() {
            (0, min_depth, min_depth)
        }
        else {
            let tmp = game.clone().unwrap();
            let mode_index = tmp.game_mode.get_index();
            let get_depth = |player: &Player| -> f32 {
                if let Player::Ia{ia, ..} = player {
                    return f32::from(ia.depth);
                }
                1.0
            };
            let first_ia_depth = match mode_index {
                0 => min_depth,
                1 => get_depth(&tmp.white_player),
                _ => get_depth(&tmp.black_player),
            };
            let second_ia_depth = match mode_index {
                3 => get_depth(&tmp.white_player),
                _ => min_depth,
            };
            (mode_index, first_ia_depth, second_ia_depth)
        };
        GameBuilder {
            mode_index,
            game_modes: [
                "Player vs Player",
                "Player vs Ia",
                "Ia vs Player",
                "Ia vs Ia",
            ],
            first_ia_depth,
            second_ia_depth,
            min_depth,
            max_depth,
            display_weight: false,
            change_window: false,
            game,
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
            }
        };

        let human_new = || -> Player {
            Player::Human
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
        if self.game.is_some() {
            Game::new_with_game(self.game.clone().unwrap(), black_player, white_player, self.game_modes[self.mode_index], self.display_weight)
        }
        else {
            Game::new(black_player, white_player, self.game_modes[self.mode_index], self.display_weight)
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
