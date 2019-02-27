use crate::WidgetIds;
use conrod::*;

pub enum PageType {
    Homepage,
    Gameboard,
}

pub trait GameViewController {
    fn new(widget_ids: &WidgetIds) -> Box<Self> where Self: Sized;
    fn show(&mut self, ui: &mut UiCell, widget_ids: &WidgetIds);
    fn get_type(&self) -> PageType;
    fn need_change_window(&self) -> bool;
}