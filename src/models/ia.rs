use crate::traits::player::*;
use crate::models::gameboard::Gameboard;

pub struct IA {
    pub depth: u8,
}

impl IA {
    pub fn new(depth: u8) -> IA {
        // println!("depth: {}", depth);
        IA {
            depth,
        }
    }
}

impl Player for IA {
    fn put_stone(&self) {

    }

    fn get_type(&self) -> PlayerType {
        PlayerType::Ia
    }

    fn set_move(&mut self, selected_move: Option<Gameboard>) {
    }

    fn get_move(&self) -> Option<Gameboard> {
        //Call_algo 
        None
    }
}

