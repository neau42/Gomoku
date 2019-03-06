use crate::traits::player::*;

pub struct IA {
    depth: u8,
    selected_move: Option<(usize, usize)>,
}

impl IA {
    pub fn new(depth: u8) -> IA {
        println!("depth: {}", depth);
        IA {
            depth,
            selected_move: None,
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
        self.selected_move = selected_move;
    }

    fn get_move(&self) -> Option<(usize, usize)> {
        self.selected_move
    }
}