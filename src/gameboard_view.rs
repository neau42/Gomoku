//! Gameboard view.

use graphics::types::Color;
use graphics::{Context, Graphics};
use graphics::character::CharacterCache;

use crate::gameboard_controller::GameboardController;

pub struct GameboardViewSettings {
    pub position: [f64; 2],
    pub size: f64,
    pub background_color: Color,
    pub border_color: Color,
	// pub click_position: [f64; 2],
	// pub release_position: [f64; 2],
}

impl GameboardViewSettings {
  pub fn new() -> GameboardViewSettings {
      GameboardViewSettings {
          position: [10.0; 2],
          size: 400.0,
          background_color: [0.8, 0.8, 1.0, 1.0],
          border_color: [0.0, 0.0, 0.2, 1.0],
		//   click_position: [10.2; 2],
		//   release_position: [10.2; 2],

      }
  }
}

pub struct GameboardView {
    pub settings: GameboardViewSettings,
}

impl GameboardView {
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView {
            settings: settings,
        }
    }
}