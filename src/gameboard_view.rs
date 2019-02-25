//! Gameboard view.

use graphics::types::Color;
use graphics::{Context, Graphics, Line, Rectangle, Ellipse};
use graphics::character::CharacterCache;

use crate::gameboard_controller::GameboardController;

pub struct GameboardViewSettings {
    pub position: [f64; 2],
    pub size: f64,
    pub background_color: Color,
	pub selected_stone_background_color: Color,
	pub preview_stone_background_color: Color,
	pub hoshi_size: f64,
	// pub click_position: [f64; 2],
	// pub release_position: [f64; 2],
}

impl GameboardViewSettings {
  pub fn new() -> GameboardViewSettings {
      GameboardViewSettings {
          position: [36.0; 2],
          size: 950.0,
          background_color: [0.8, 0.8, 1.0, 1.0],
          selected_stone_background_color: [0.8, 0.8, 1.0, 1.0],
          preview_stone_background_color: [0.8, 0.8, 1.0, 0.5],
		  hoshi_size: 15.0,
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

	    /// Draw gameboard.
    pub fn draw<G: Graphics, C>(
        &self,
        controller: &GameboardController,
        glyphs: &mut C,
        c: &Context,
        g: &mut G
    )
        where C: CharacterCache<Texture = G::Texture>
    {

        let ref settings = self.settings;

	  // Draw lines
        let stone_edge = Line::new([0.0, 0.0, 0.2, 1.0],1.0);
        for i in 0..19 {
            let x = settings.position[0] + i as f64 / 18.0 * settings.size;
            let y = settings.position[1] + i as f64 / 18.0 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            stone_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.position[0], y, x2, y];
            stone_edge.draw(hline, &c.draw_state, c.transform, g);
		}
	// Draw Hoshi
		for i in 0..3 {
			for j in [3, 9, 16].iter() {
				Rectangle::new([0.0, 0.0, 0.0, 1.0])
				.draw([
					settings.position[0] + *j as f64 * (settings.size / 18.0) - settings.hoshi_size / 2.0,
					settings.position[1] + (3.0 + (6.0 * i as f64)) * (settings.size / 18.0) - settings.hoshi_size / 2.0,
					settings.hoshi_size, settings.hoshi_size], &c.draw_state, c.transform, g);
				}
			}
	// Draw selected stone
		if let Some(ind) = controller.selected_stone {
			draw_one_stone(settings, ind, settings.selected_stone_background_color, c, g);
		}
	// Draw preview stone
		if let Some(ind) = controller.preview_stone {
			draw_one_stone(settings, ind, settings.preview_stone_background_color, c, g);
		}
	}
}

pub fn draw_one_stone<G: Graphics>(
	board: &GameboardViewSettings,
	ind: [usize; 2],
	color: Color, 
	c: &Context,
	g: &mut G
	)
	{
	let stone_size = board.size / 18.0;
	let pos = [ind[0] as f64 * stone_size - stone_size / 2.0,
		ind[1] as f64 * stone_size - stone_size / 2.0];
	let stone_position = [
		board.position[0] + pos[0], board.position[1] + pos[1],
		stone_size, stone_size
	];
	Ellipse::new(color)
		.draw(stone_position, &c.draw_state, c.transform, g);
}
