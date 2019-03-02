use crate::WidgetIds;
use crate::traits::view_model::*;
use conrod::backend::glium::glium::glutin::*;
use conrod::*;

#[derive(PartialEq)]
pub enum PageType {
    Homepage,
    Gameboard,
}

pub trait GameViewController {
    fn new(widget_ids: &WidgetIds) -> Box<Self> where Self: Sized;
    fn show(&self, model: &mut Box<dyn GameViewModel>, ui: &mut UiCell, widget_ids: &WidgetIds);
	fn check_event(&mut self, event: &Event, model: &mut Box<dyn GameViewModel>, ui: &mut UiCell, widget_ids: &WidgetIds);
    fn get_type(&self) -> PageType;
}