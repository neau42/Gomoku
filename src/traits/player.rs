use crate::models::gameboard::Gameboard;

#[derive(PartialEq)]
pub enum PlayerType {
    Human,
    Ia,
}

pub trait Player {
    fn put_stone(&self);
    fn get_type(&self) -> PlayerType;
    fn set_move(&mut self, selected_move: Option<Gameboard>);
    fn get_move(&self) -> Option<Gameboard>;
}