//! Gameboard view.

use graphics::types::Color;
use graphics::{Context, Graphics, Line, Rectangle, Ellipse};
use graphics::character::CharacterCache;

use crate::controllers::gameboard::GameboardController;
use crate::models::gameboard::Stone;

pub struct GameboardViewSettings {
    pub position: [f64; 2],
    pub size: f64,
    pub background_color: Color,
	pub selected_stone_background_color: Color,
	pub preview_stone_background_color: Color,
	pub hoshi_size: f64,
}

impl GameboardViewSettings {
  pub fn new() -> GameboardViewSettings {
      GameboardViewSettings {
          position: [36.0; 2],
          size: 950.0,
          background_color: [0.8, 0.8, 1.0, 1.0],
          selected_stone_background_color: [0.9, 0.9, 0.9, 1.0],
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
        &self, controller: &GameboardController, _glyphs: &mut C, c: &Context, g: &mut G
    )
    where C: CharacterCache<Texture = G::Texture> {
		let ref settings = self.settings;
		draw_map(settings, controller, c, g);
		draw_stones(settings, controller, c, g);

		// Draw preview stone
		if let Some(ind) = controller.preview_stone {
			draw_one_stone(settings, controller, ind, settings.preview_stone_background_color, c, g);
		}
	}
}

pub fn draw_map<G: Graphics>(settings: &GameboardViewSettings, controller: &GameboardController, c: &Context, g: &mut G)
{
	draw_lines(settings, controller, c, g);
	draw_hoshi(settings, controller, c, g);
}

// Draw stone on map
pub fn draw_stones<G: Graphics>(settings: &GameboardViewSettings, controller: &GameboardController, c: &Context, g: &mut G) {
	let map_size = controller.gameboard.size;

	for i in 0..map_size * map_size {
		match controller.gameboard.cells[i/map_size][i%map_size] {
			Stone::WHITE =>	draw_one_stone(settings, controller, [i/map_size, i%map_size], [1.0, 1.0, 1.0, 1.0], c, g),
			Stone::BLACK => draw_one_stone(settings, controller, [i/map_size, i%map_size], [0.0, 0.0, 0.0, 1.0], c, g),
			_ => (),
		}
	}
}

pub fn draw_lines<G: Graphics>(settings: &GameboardViewSettings, controller: &GameboardController, c: &Context, g: &mut G) {
   let stone_edge = Line::new([0.0, 0.0, 0.0, 1.0],0.5);
   let stone_edge_border = Line::new([0.9, 0.9, 0.8, 0.6],0.5);
	let map_size = controller.gameboard.size;

	//draw line border horizontal
	for i in 0..map_size {
		let y = settings.position[1] + i as f64 / (map_size - 1) as f64 * settings.size;
		let x2 = settings.position[0] + settings.size;

		let hline_border = [settings.position[0], y + 1.0, x2, y + 1.0];
		stone_edge_border.draw(hline_border, &c.draw_state, c.transform, g);
	}
	//draw line horizontal and vertical
	for i in 0..map_size {
		let x = settings.position[0] + i as f64 / (map_size - 1) as f64 * settings.size;
		let y = settings.position[1] + i as f64 / (map_size - 1) as f64 * settings.size;
		let x2 = settings.position[0] + settings.size;
		let y2 = settings.position[1] + settings.size;

		let hline = [settings.position[0], y, x2, y];
		stone_edge.draw(hline, &c.draw_state, c.transform, g);
		let vline = [x, settings.position[1], x, y2];
		stone_edge.draw(vline, &c.draw_state, c.transform, g);
	}
}

pub fn draw_hoshi<G: Graphics>(settings: &GameboardViewSettings, controller: &GameboardController, c: &Context, g: &mut G)
{
	let map_size = controller.gameboard.size;
	for i in [3, (map_size - 1) / 2, map_size - 4].iter() {
		for j in [3, (map_size - 1) / 2, map_size - 4].iter() {
			Rectangle::new([0.0, 0.0, 0.0, 1.0])
			.draw([
				settings.position[0] + *j as f64 * (settings.size / (map_size - 1) as f64) - settings.hoshi_size / 2.0,
				settings.position[1] + *i as f64 * (settings.size / (map_size - 1) as f64) - settings.hoshi_size / 2.0,
				settings.hoshi_size,
				settings.hoshi_size],
				&c.draw_state, c.transform, g);
		}
	}
}

pub fn draw_one_stone<G: Graphics>(
	board: &GameboardViewSettings,
	controller: &GameboardController,
	ind: [usize; 2], color: Color, c: &Context, g: &mut G) {
	let map_size = controller.gameboard.size;

		
	let stone_size = board.size / (map_size - 1) as f64;
	
	let pos = [ind[0] as f64 * stone_size - stone_size / 2.0,
		ind[1] as f64 * stone_size - stone_size / 2.0];

	let stone_position = [
		board.position[0] + pos[0], board.position[1] + pos[1],
		stone_size, stone_size];
	Ellipse::new(color)
	.draw(stone_position, &c.draw_state, c.transform, g);
}
