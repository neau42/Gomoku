use std::any::Any;

pub trait GameViewModel {
    fn get_model(&mut self) -> &mut dyn Any;
    fn need_change_window(&self) -> bool;
}