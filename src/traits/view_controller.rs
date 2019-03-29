use crate::WidgetIds;
use crate::traits::view_model::*;
use conrod::*;

#[derive(PartialEq)]
pub enum PageType {
    GameBuilder,
    Game,
}

pub trait GameViewController {
    fn new(widget_ids: &WidgetIds) -> Box<Self> where Self: Sized;
    fn show(&mut self, model: &mut dyn GameViewModel, ui: &mut UiCell, widget_ids: &WidgetIds);
    fn get_type(&self) -> PageType;
}