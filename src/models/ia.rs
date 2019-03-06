use crate::traits::player::*;

pub struct IA {
    depth: u8,
}

impl IA {
    pub fn new(depth: u8) -> IA {
        println!("depth: {}", depth);
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

    fn set_move(&mut self, selected_move: Option<(usize, usize)>) {
    }

    fn get_move(&self) -> Option<(usize, usize)> {
        //Call_algo 
        None
    }
}