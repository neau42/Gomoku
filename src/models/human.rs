use crate::traits::player::*;
use crate::models::gameboard::Gameboard;

pub struct Human {
    selected_move: Option<Gameboard>,
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

    fn set_move(&mut self, selected_move: Option<Gameboard>) {
        self.selected_move = selected_move;
    }

    fn get_move(&self) -> Option<Gameboard> {
        self.selected_move
    }

}