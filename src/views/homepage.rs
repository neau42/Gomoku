use conrod::*;
use ::image::open;
use std::path::Path;

// use conrod::widget::Id;
use conrod::image::Id;
use glium::backend::Facade;
widget_ids! {pub struct Ids {
    text,
    background
}}

pub struct HomepageView {
    pub ids: Ids,
}

impl HomepageView {
    pub fn new(ui: &mut Ui) -> HomepageView {
        HomepageView {
            ids: Ids::new(ui.widget_id_generator())
        }
    }

    pub fn display(&self, ui: &mut UiCell, image_map: &mut conrod::image::Map::<glium::texture::Texture2d>, display: &Facade) {

        widget::Text::new("Hello World!")
            .middle_of(ui.window)
            .color(conrod::color::WHITE)
            .font_size(32)
            .set(self.ids.text, ui);
    }
}