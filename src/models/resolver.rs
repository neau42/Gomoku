//! Game board logic.
use crate::traits::view_model::*;
use crate::models::gameboard::*;
use std::any::Any;


pub struct Resolver {
	pub state: Gameboard,
}

/// Creates a new game board.
impl Resolver {
	pub fn new() -> Resolver {
		Resolver {
			state: Gameboard::new(),
		}
	}
}

impl GameViewModel for Resolver {
    fn get_model(&mut self) -> &mut dyn Any {
        self
    }

    fn need_change_window(&self) -> bool {
		false
	}
}