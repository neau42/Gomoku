//! Gameboard controller.

use piston::input::{GenericEvent, Button, Key, MouseButton};

use crate::Gameboard;

pub struct GameboardController {
    pub gameboard: Gameboard,
    cursor_pos: [f64; 2],
    click_position: [f64; 2],
    release_position: [f64; 2],
}

impl GameboardController {
  pub fn new(gameboard: Gameboard) -> GameboardController {
      GameboardController {
          gameboard: gameboard,
          cursor_pos: [0.0; 2],
		  click_position: [0.0; 2],
		  release_position: [0.0; 2],
      }
  }

	pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {
    if let Some(pos) = e.mouse_cursor_args() {
	    self.cursor_pos = pos;
	// 	// println!("mouse move pos: {:?}", self.cursor_pos);
	}
	if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
		println!("mouse click pos: {:?}", self.cursor_pos);
		self.click_position = self.cursor_pos;
  	}
  	if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
		self.release_position = self.cursor_pos;
		println!("mouse release_args pos: {:?}", self.cursor_pos);
  	}
	if let Some(Button::Keyboard(key)) = e.press_args() {
		println!("keyboard! pressed: {:?}", key);
	}
  }
}