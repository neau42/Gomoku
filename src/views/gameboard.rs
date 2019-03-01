//! Gameboard view.

use graphics::types::Color;
use graphics::{Context, Graphics, Line, Rectangle, Ellipse};
use crate::controllers::gameboard::GameboardController;
use crate::models::gameboard::Stone;

pub struct GameboardView {
    pub position: [f64; 2],
    pub size: f64,
    pub background_color: Color,
	pub selected_stone_background_color: Color,
	pub preview_stone_background_color: Color,
	pub hoshi_size: f64,
}

impl GameboardView {
	pub fn new() -> GameboardView {
		GameboardView {
			position: [36.0; 2],
			size: 950.0,
			background_color: [0.8, 0.8, 1.0, 1.0],
			selected_stone_background_color: [0.9, 0.9, 0.9, 1.0],
			preview_stone_background_color: [0.8, 0.8, 1.0, 0.5],
			hoshi_size: 15.0,
		}
	}

	/// Draw gameboard.
    pub fn draw<G: Graphics>(
        &self, controller: &GameboardController, c: &Context, g: &mut G)
    {
		draw_map(&self, controller, c, g);
		draw_stones(&self, controller, c, g);

		// Draw preview stone
		if let Some(ind) = controller.preview_stone {
			draw_one_stone(&self, controller, ind, self.preview_stone_background_color, c, g);
		}
	}
}

pub fn draw_map<G: Graphics>(view: &GameboardView, controller: &GameboardController, c: &Context, g: &mut G)
{
	draw_lines(view, controller, c, g);
	draw_hoshi(view, controller, c, g);
}

// Draw stones on map
pub fn draw_stones<G: Graphics>(view: &GameboardView, controller: &GameboardController, c: &Context, g: &mut G) {
	let map_size = controller.gameboard.size;

	for i in 0..map_size * map_size {
		match controller.gameboard.cells[i/map_size][i%map_size] {
			Stone::WHITE =>	draw_one_stone(view, controller, [i/map_size, i%map_size], [1.0, 1.0, 1.0, 1.0], c, g),
			Stone::BLACK => draw_one_stone(view, controller, [i/map_size, i%map_size], [0.0, 0.0, 0.0, 1.0], c, g),
			_ => (),
		}
	}
}

pub fn draw_lines<G: Graphics>(view: &GameboardView, controller: &GameboardController, c: &Context, g: &mut G) {
   let stone_edge = Line::new([0.0, 0.0, 0.0, 1.0],0.5);
   let stone_edge_border = Line::new([0.9, 0.9, 0.8, 0.6],0.5);
	let map_size = controller.gameboard.size;

	//draw lines border horizontal
	for i in 0..map_size {
		let y = view.position[1] + i as f64 / (map_size - 1) as f64 * view.size;
		let x2 = view.position[0] + view.size;

		let hline_border = [view.position[0], y + 1.0, x2, y + 1.0];
		stone_edge_border.draw(hline_border, &c.draw_state, c.transform, g);
	}
	//draw lines horizontal and vertical
	for i in 0..map_size {
		let x = view.position[0] + i as f64 / (map_size - 1) as f64 * view.size;
		let y = view.position[1] + i as f64 / (map_size - 1) as f64 * view.size;
		let x2 = view.position[0] + view.size;
		let y2 = view.position[1] + view.size;

		let hline = [view.position[0], y, x2, y];
		stone_edge.draw(hline, &c.draw_state, c.transform, g);
		let vline = [x, view.position[1], x, y2];
		stone_edge.draw(vline, &c.draw_state, c.transform, g);
	}
}

pub fn draw_hoshi<G: Graphics>(view: &GameboardView, controller: &GameboardController, c: &Context, g: &mut G)
{
	let map_size = controller.gameboard.size;
	for i in [3, (map_size - 1) / 2, map_size - 4].iter() {
		for j in [3, (map_size - 1) / 2, map_size - 4].iter() {
			Rectangle::new([0.0, 0.0, 0.0, 1.0])
			.draw([
				view.position[0] + *j as f64 * (view.size / (map_size - 1) as f64) - view.hoshi_size / 2.0,
				view.position[1] + *i as f64 * (view.size / (map_size - 1) as f64) - view.hoshi_size / 2.0,
				view.hoshi_size,
				view.hoshi_size],
				&c.draw_state, c.transform, g);
		}
	}
}

pub fn draw_one_stone<G: Graphics>(
	view: &GameboardView,
	controller: &GameboardController,
	ind: [usize; 2], color: Color, c: &Context, g: &mut G) {
	let map_size = controller.gameboard.size;

	let stone_size = view.size / (map_size - 1) as f64;
	
	let pos = [ind[0] as f64 * stone_size - stone_size / 2.0,
		ind[1] as f64 * stone_size - stone_size / 2.0];

	let stone_position = [view.position[0] + pos[0],
		view.position[1] + pos[1],
		stone_size,
		stone_size];
	Ellipse::new(color)
	.draw(stone_position, &c.draw_state, c.transform, g);
}
