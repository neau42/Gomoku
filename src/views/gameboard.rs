//! Gameboard view.

// use graphics::types::Color;
// use graphics::{Context, Graphics, Line, Rectangle, Ellipse};
use crate::controllers::gameboard::GameboardController;
use crate::models::gameboard::Stone;
use crate::models::gameboard::Gameboard;
use crate::views::widget_gameboard;

use crate::models::game_info::*;
use crate::WidgetIds;
use conrod::color::Rgba;
use conrod::*;
// use conrod::backend::glium::glium::*;


pub struct GameboardView {
    pub position: [f64; 2],
    pub size: f64,
    // pub background_color: Rgba,
	// pub selected_stone_background_color: Rgba,
	// pub preview_stone_background_color: Rgba,
	pub hoshi_size: f64,
}

impl GameboardView {
	pub fn new() -> GameboardView {
		GameboardView {
			position: [36.0; 2],
			size: 800.0,
			// background_color: Rgba(0.8, 0.8, 1.0, 1.0),
			// selected_stone_background_color: Rgba(0.9, 0.9, 0.9, 1.0),
			// preview_stone_background_color: Rgba(0.8, 0.8, 1.0, 0.5),
			hoshi_size: 15.0,
		}
	}

    pub fn display_grid(&self, model: &mut Gameboard, ui: &mut UiCell, widget_ids: &WidgetIds) {
	model.cells[0][0] = Stone::WHITE;
	model.cells[2][2] = Stone::BLACK;
	model.cells[2][3] = Stone::BLACK;
	model.cells[3][2] = Stone::WHITE;
	model.cells[3][3] = Stone::BLACK;

		if let Some((y, x)) = widget_gameboard::Board::new(model)
		// if widget_gameboard::Board::new(model)
			.middle_of(widget_ids.background)
			.down_from(widget_ids.title, 15.0)
			.w_h(self.size, self.size)
			.set(widget_ids.grid, ui)
			.was_clicked() {
				// println!("click !!!!!");
				println!("click [{}][{}]!!!!!", y, x);
			}
	}
}

		// let mut elements = widget::Matrix::new(model.size, model.size)
    	//     .w_h(self.size - 1.0, self.size - 1.0)
		// 	.x(self.position[0])
		// 	.y(self.position[1])
		// 	// .color(color::BLACK)
       	// 	.set(widget_ids.grid, ui);
		// 	// .down_from(widget_ids.title, 15.0)
    	//     // .mid_top_of(widget_ids.footer)
		// // while let Some(elem) = elements.next(ui) {
		// // 	let (r, c) = (elem.row, elem.col);
		// // 	let button = widget::Button::new().color(color::TRANSPARENT);
		// // 	for _click in elem.set(button, ui) {
		// // 		println!("Click on {:?}", (r, c));
		// // 	}
		

    	// let mut elements = widget::Matrix::new(model.size, model.size)
    	//     .w_h(self.size, self.size)
		// 	.x_y_relative(22.5, 22.5)
		// 	// .color(color::TRANSPARENT);
		// 	// .down_from(widget_ids.title, 15.0)
    	//     // .mid_top_of(widget_ids.footer)
       	// 	.set(widget_ids.grid_select, ui);
		// while let Some(elem) = elements.next(ui) {
		// 	let (r, c) = (elem.row, elem.col);
		// 	let button = widget::Button::new().color(color::TRANSPARENT);
		// 	for _click in elem.set(button, ui) {
		// 		println!("Click on {:?}", (r, c));
		// 	}
		// }


// }
//    let stone_edge_style = conrod::widget::primitive::line::Style::new();
//    stone_edge_style.color(conrod::color::BLACK);

//    let stone_edge_border_style = conrod::widget::primitive::line::Style::new();
//    stone_edge_border_style.color(conrod::color::WHITE);

// 	for i in 0..model.size {
// 		let y = self.position[1] + i as f64 / (model.size - 1) as f64 * self.size;
// 		let x2 = self.position[0] + self.size;


// 		  let stone_edge_border = conrod::widget::primitive::line::Line::styled([self.position[0], y + 1.0], [x2, y + 1.0], stone_edge_border_style);


		// stone_edge_border.draw(hline_border, &c.draw_state, c.transform, g);

	//    let stone_edge_border = widget::primitive::line::Line::styled([self.position[0], y + 1.0], [x2, y + 1.0], stone_edge_border_style);
	// println!("x0: {} y0: {}, x1: {} y1: {}", self.position[0], y + 1.0, x2, y + 1.0);

		// widget::primitive::line::Line::centred([self.position[0], y + 1.0], [x2, y + 1.0])
		// widget::primitive::line::Line::centred([0.0, 0.0], [500.0, 500.0])
		// .color(conrod::color::BLACK)
        // .middle_of(widget_ids.title)
		// .down_from(widget_ids.title, 15.0)
		// .set(widget_ids.title, ui);
		// 	let lines = widget::grid::Lines::step(1.0_f64).thickness(0.5);
		// let lines = &[
		// 	lines.x(),
		// 	lines.y(),
		// ];
		// widget::Grid::new(0.0, model.size as f64 - 1.0, 0.0, model.size as f64 - 1.0, lines.iter().cloned())
		// 	.wh([855.0, 855.0])
		// 	.color(color::BLACK)
        //     .middle_of(widget_ids.title)
        //     .down_from(widget_ids.title, 15.0)
		// 	.set(widget_ids.grid, ui);

		// stone_edge_border.draw(hline_border, &c.draw_state, c.transform, g);
	// }
	//draw lines horizontal and vertical
	// for i in 0..model.size {
	// 	let x = self.position[0] + i as f64 / (model.size - 1) as f64 * self.size;
	// 	let y = self.position[1] + i as f64 / (model.size - 1) as f64 * self.size;
	// 	let x2 = self.position[0] + self.size;
	// 	let y2 = self.position[1] + self.size;

	// 	let hline = [self.position[0], y, x2, y];
	//    let stone_edge = conrod::widget::primitive::line::Line::styled([self.position[0], y], [x2, y], stone_edge_style);

	// 	// stone_edge.draw(hline, &c.draw_state, c.transform, g);
	// 	let vline = [x, self.position[1], x, y2];
	//    let stone_edge = conrod::widget::primitive::line::Line::styled([x, self.position[1]], [x, y2], stone_edge_style);

	// }
	// 	let lines = widget::grid::Lines::step(1.0_f64).thickness(0.5);
	// 	let lines = &[
	// 		lines.x(),
	// 		lines.y(),
	// 	];

	// 	widget::Grid::new(0.0, model.size as f64 - 1.0, 0.0, model.size as f64 - 1.0, lines.iter().cloned())
	// 		.wh([855.0, 855.0])
	// 		.color(color::BLACK)
    //         .middle_of(widget_ids.title)
    //         .down_from(widget_ids.title, 15.0)
	// 		.set(widget_ids.grid, ui);
	// }


    // pub fn display_canvas(&self, ui: &mut UiCell, widget_ids: &WidgetIds) {
    //     widget::Canvas::new()
    //         .w(500.0)
    //         .border(0.0)
    //         .middle_of(widget_ids.window_canvas)
    //         .down_from(widget_ids.title, 50.0)
    //         .color(color::RED)
    //         .set(widget_ids.homepage_canvas, ui);
    // }
// }

// 	/// Draw gameboard.
//     pub fn draw<G: Graphics>(
//         &self, controller: &GameboardController, c: &Context, g: &mut G)
//     {
// 		draw_map(&self, controller, c, g);
// 		draw_stones(&self, controller, c, g);

// 		// Draw preview stone
// 		if let Some(ind) = controller.preview_stone {
// 			draw_one_stone(&self, controller, ind, self.preview_stone_background_color, c, g);
// 		}
// 	}
// }

	// pub fn draw_map(&self, model: &mut Gameboard, ui: &mut UiCell, widget_ids: &WidgetIds)
	// {
	// 	println!("draw_map!");
	// 	draw_lines(self, model.size);
	// 	// draw_hoshi(view, controller, c, g);
	// }



// pub fn draw_lines(view: &GameboardView, size: usize) {
//    let stone_edge_style = conrod::widget::primitive::line::Style::new();
//    stone_edge_style.color(conrod::color::BLACK);

//    let stone_edge_border_style = conrod::widget::primitive::line::Style::new();
//    stone_edge_border_style.color(conrod::color::WHITE);

//    stone_edge_border_style.color([0.9, 0.9, 0.8, 0.6]);


//    let stone_edge_border = conrod::widget::primitive::line::Line::new([0.9, 0.9, 0.8, 0.6],0.5);

	//draw lines border horizontal
// 	for i in 0..size {
// 		let y = view.position[1] + i as f64 / (size - 1) as f64 * view.size;
// 		let x2 = view.position[0] + view.size;

// 	   let stone_edge_border = conrod::widget::primitive::line::Line::styled([view.position[0], y + 1.0], [x2, y + 1.0], stone_edge_border_style);


// 		// stone_edge_border.draw(hline_border, &c.draw_state, c.transform, g);
// 	}
// 	//draw lines horizontal and vertical
// 	for i in 0..size {
// 		let x = view.position[0] + i as f64 / (size - 1) as f64 * view.size;
// 		let y = view.position[1] + i as f64 / (size - 1) as f64 * view.size;
// 		let x2 = view.position[0] + view.size;
// 		let y2 = view.position[1] + view.size;

// 		let hline = [view.position[0], y, x2, y];
// 	   let stone_edge = conrod::widget::primitive::line::Line::styled([view.position[0], y], [x2, y], stone_edge_style);

// 		// stone_edge.draw(hline, &c.draw_state, c.transform, g);
// 		let vline = [x, view.position[1], x, y2];
// 	   let stone_edge = conrod::widget::primitive::line::Line::styled([x, view.position[1]], [x, y2], stone_edge_style);

// 	}
// }



/*
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
*/