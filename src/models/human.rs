use crate::traits::player::*;
use crate::models::gameboard::Gameboard;

pub struct Human {
    selected_move: Option<(usize, usize)>,
}

impl Human {
    pub fn new() -> Human {
        Human {
            selected_move: None
        }
    }
}

impl Player for Human {
    fn put_stone(&self) {

    }

    fn get_type(&self) -> PlayerType {
        PlayerType::Human
    }

    fn set_move(&mut self, selected_move: Option<(usize, usize)>) {
        self.selected_move = selected_move;
    }

    fn get_move(&self) -> Option<(usize, usize)> {
        self.selected_move
    }
}