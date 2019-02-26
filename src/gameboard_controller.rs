//! Gameboard controller.

use piston::input::{GenericEvent, Button, MouseButton};

use crate::Gameboard;
use crate::gameboard::Stone;

pub struct GameboardController {
    pub gameboard: Gameboard,
    cursor_pos: [f64; 2],
    click_position: [f64; 2],
    release_position: [f64; 2],
	pub selected_stone: Option<[usize; 2]>,
	pub preview_stone: Option<[usize; 2]>,

	test: bool,
}

impl GameboardController {
  pub fn new(gameboard: Gameboard) -> GameboardController {
      GameboardController {
          gameboard: gameboard,
          cursor_pos: [0.0; 2],
		  click_position: [0.0; 2],
		  release_position: [0.0; 2],
		  selected_stone: None,
		  preview_stone: None,

		  test: true,
      }
  }

	pub fn event<E: GenericEvent>(&mut self, size: f64, e: &E) {
    if let Some(pos) = e.mouse_cursor_args() {
	    self.cursor_pos = pos;
        self.preview_stone = get_stone(self.cursor_pos[0], self.cursor_pos[1],size);
	// 	// println!("mouse move pos: {:?}", self.cursor_pos);
	}
	if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
		println!("mouse click pos: {:?}", self.cursor_pos);
		self.click_position = self.cursor_pos;
  	}
  	if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
        self.selected_stone = get_stone(self.cursor_pos[0], self.cursor_pos[1],size);
		match self.selected_stone {
			Some(x_y) => {
				println!("controller: X: {}, Y: {}", x_y[0], x_y[1]);
				if self.test == true {
					self.gameboard.cells[x_y[0]][x_y[1]] = Stone::BLACK; }
				else {
					self.gameboard.cells[x_y[0]][x_y[1]] = Stone::WHITE; }
				self.test = !self.test;
		},
			None => (),
		}
		self.release_position = self.cursor_pos;
		println!("mouse release_args pos: {:?}", self.cursor_pos);
  	}
	if let Some(Button::Keyboard(key)) = e.press_args() {
		println!("keyboard! pressed: {:?}", key);
	}
  }
}

pub fn get_stone(x: f64, y: f64, size: f64) -> Option<[usize; 2]> {
	// Check that coordinates are inside board boundaries.
	 if x >= 36.0 - 25.0 && x < size + 36.0 + 25.0 && y >= 36.0 - 25.0 && y < size + 36.0 + 25.0 {
		let stone_x = ((x - 36.0 + 25.0) / size * 18.0) as usize;
		let stone_y = ((y - 36.0 + 25.0) / size * 18.0) as usize;
		// println!("stone_x: {}, stone_y: {}", stone_x, stone_y);
		return Some([stone_x, stone_y])
	 }
	 None
}