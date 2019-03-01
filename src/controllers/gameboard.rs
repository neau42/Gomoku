//! Gameboard controller.

use std::collections::HashMap;
use conrod::widget::id::Id;
use crate::WidgetIds;

use graphics::{Context, Graphics};

use piston::input::{GenericEvent, Button, MouseButton};

use crate::views::gameboard::GameboardView;
use crate::models::gameboard::Gameboard;
use crate::models::gameboard::Stone;

pub struct GameboardController {
    pub gameboard: Gameboard,
	pub view: GameboardView,
    cursor_pos: [f64; 2],
    click_position: [f64; 2],
    release_position: [f64; 2],
	pub selected_stone: Option<[usize; 2]>,
	pub preview_stone: Option<[usize; 2]>,
	events: HashMap<Id, fn()>,


	test_switch_player: bool,
}

impl GameboardController {
	pub fn new(gameboard: Gameboard, view: GameboardView, widget_ids: &WidgetIds) -> GameboardController {
		let mut gameboard = GameboardController {
			gameboard,
			view,
			cursor_pos: [0.0; 2],
			click_position: [0.0; 2],
			release_position: [0.0; 2],
			selected_stone: None,
			preview_stone: None,
			events: HashMap::new(),

			test_switch_player: false,
			};
	  gameboard.set_events(widget_ids);
	  gameboard
  }
		fn set_events(&mut self, widget_ids: &WidgetIds) {
			self.events.insert(widget_ids.button_player_vs_player, || {
            	println!("click on button 'player vs player'");
        	});
		}

	pub fn event<E: GenericEvent>(&mut self, e: &E) {
    if let Some(pos) = e.mouse_cursor_args() {
	    self.cursor_pos = pos;
        self.preview_stone = self.get_cell(self.cursor_pos[0], self.cursor_pos[1]);
	}
	if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
		// println!("mouse click pos: {:?}", self.cursor_pos);
		self.click_position = self.cursor_pos;
  	}
  	if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
		self.put_stone();
    	self.release_position = self.cursor_pos;
		// println!("mouse release_args pos: {:?}", self.cursor_pos);
  	}
	if let Some(Button::Keyboard(key)) = e.press_args() {
		// println!("keyboard! pressed: {:?}", key);
	}
  }
  pub fn put_stone(&mut self) -> bool {
		self.selected_stone = self.get_cell(self.cursor_pos[0], self.cursor_pos[1]);
		match self.selected_stone {
			Some(x_y) => {
				if self.gameboard.cells[x_y[0]][x_y[1]] == Stone::NOPE {
					if self.test_switch_player == true { //TEMP
						self.gameboard.cells[x_y[0]][x_y[1]] = Stone::BLACK; }
					else {
						self.gameboard.cells[x_y[0]][x_y[1]] = Stone::WHITE; }
					self.test_switch_player = !self.test_switch_player;
					println!("controller: X: {}, Y: {}", x_y[0], x_y[1]);
					return true;
				}
			},
			None => (),
		}
		false
  }
	pub fn get_cell(&mut self, x: f64, y: f64) -> Option<[usize; 2]> {
		let size_px = self.view.size;
		let map_position = self.view.position;
		let map_size = self.gameboard.size;
		let semi_cell_size = size_px / map_size as f64 / 2.0;

		// Check that coordinates are inside board boundaries.
		if x >= map_position[0] - semi_cell_size
			&& x < size_px + map_position[0] + semi_cell_size
			&& y >= map_position[1] - semi_cell_size
			&& y < size_px + map_position[1] + semi_cell_size {
			let stone_x = ((x - map_position[0] + semi_cell_size) / size_px * (map_size - 1) as f64) as usize;
			let stone_y = ((y - map_position[1] + semi_cell_size) / size_px * (map_size - 1) as f64) as usize;
			// println!("stone_x: {}, stone_y: {}", stone_x, stone_y);
			return Some([stone_x, stone_y])
		}
		None
	}
	    pub fn show<G: Graphics>(&self, context: &Context, graphic: &mut G) {
			self.view.draw(&self, &context, graphic);
	}

}
