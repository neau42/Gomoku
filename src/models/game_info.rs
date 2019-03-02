use crate::traits::view_model::*;
use std::string::ToString;
use std::any::Any;

#[derive(PartialEq, Clone, Copy, IntoStaticStr, EnumIter, ToString)]
pub enum GameMode {
    PlayerVsPlayer,
    PlayerVsIa,
    IaVsIa
}

pub struct GameInfo {
    pub mode_index: Option<usize>,
    pub game_modes: [String; 3],
    pub first_ia_depth: f32,
    pub second_ia_depth: f32,
    pub min_depth: f32,
    pub max_depth: f32,
    pub display_weight: bool,
    change_window: bool,
}

impl GameInfo {
     pub fn new() -> GameInfo {
        let min_depth = 0.0 as f32;
        let max_depth = 10.0 as f32;
        GameInfo {
            mode_index: Some(0),
            game_modes: [GameMode::PlayerVsPlayer.to_string(), GameMode::PlayerVsIa.to_string(), GameMode::IaVsIa.to_string()],
            first_ia_depth: min_depth,
            second_ia_depth: min_depth,
            min_depth,
            max_depth,
            display_weight: false,
            change_window: false,
        }
    }

    pub fn set_mode(&mut self, mode_index: Option<usize>) {
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
}

impl GameViewModel for GameInfo {
    fn get_model(&mut self) -> &mut dyn Any {
		println!("get_model GameInfo");
        self
    }

    fn need_change_window(&self) -> bool {
        self.change_window
    }
}