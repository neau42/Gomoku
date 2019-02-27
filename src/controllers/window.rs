use conrod::UiCell;
use crate::WidgetIds;
use crate::views::window::WindowView;
use crate::models::window::WindowModel;
use conrod::image::Map;
use ::image::open;
use conrod::*;
use glium::texture::*;
use std::path::Path;

pub struct WindowController {
    pub view: WindowView,
    pub model: WindowModel,
}

impl WindowController {
    pub fn new() -> WindowController {
        let view = WindowView::new();
        let model = WindowModel::new();
        WindowController {
            view,
            model
        }
    }

    pub fn load_background(&mut self, image_map: &mut Map<Texture2d>, display: &glium::Display) {
        let rgba_image = open(&Path::new("assets/images/wood.jpg")).unwrap().to_rgba();
        let image_dimensions = rgba_image.dimensions();
        let raw_image = RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), image_dimensions);
        let texture = Texture2d::new(display, raw_image).unwrap();

        self.model.background = Some(image_map.insert(texture));
    }

    pub fn show(&self, ui: &mut UiCell, widget_ids: &WidgetIds) {
        if self.model.background.is_some() {
            self.view.display_background(ui, widget_ids, self.model.background.unwrap());
        }
        self.view.display_canvas(ui, widget_ids);
        self.view.display_title(ui, widget_ids);
    }
}
