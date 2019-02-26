use crate::WidgetIndex;
use conrod::*;
use ::image::open;
use std::path::Path;
use conrod::image::Id;
use glium::backend::Facade;

pub struct HomepageView {
}

impl HomepageView {
    pub fn new(ui: &mut Ui) -> HomepageView {
        HomepageView {
        }
    }

    pub fn display(&self, ui: &mut UiCell, widget_index: &WidgetIndex) {

        widget::Text::new("Hello World!")
            .middle_of(ui.window)
            .color(conrod::color::WHITE)
            .font_size(32)
            .set(widget_index.text, ui);
    }
}