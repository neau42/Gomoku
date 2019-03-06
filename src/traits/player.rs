#[derive(PartialEq)]
pub enum PlayerType {
    Human,
    Ia,
}

pub trait Player {
    fn put_stone(&self);
    fn get_type(&self) -> PlayerType;
    fn set_move(&mut self, selected_move: Option<(usize, usize)>);
    fn get_move(&self) -> Option<(usize, usize)>;
}