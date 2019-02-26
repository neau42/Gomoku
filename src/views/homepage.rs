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

        widget::Canvas::new()
            .w(500.0)
            .border(0.0)
            .middle_of(widget_index.window_canvas)
            .down_from(widget_index.title, 50.0)
            .color(color::TRANSPARENT)
            .set(widget_index.homepage_canvas, ui);

        if widget::Button::new()
            .w_of(widget_index.homepage_canvas)
            .h(75.0)
            .mid_top_of(widget_index.homepage_canvas)
            .down_from(widget_index.title, 50.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label("Player vs Player")
            .set(widget_index.button_player_vs_player, ui)
            .was_clicked()
        {
        }

        if widget::Button::new()
            .w_of(widget_index.homepage_canvas)
            .h(75.0)
            .middle_of(widget_index.homepage_canvas)
            .down_from(widget_index.button_player_vs_player, 25.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label("Player vs IA")
            .set(widget_index.button_player_vs_ia, ui)
            .was_clicked()
        {
        }

        if let Some(value) = widget::Toggle::new(true)
            .w_of(widget_index.homepage_canvas)
            .h(75.0)
            .middle_of(widget_index.homepage_canvas)
            .down_from(widget_index.button_player_vs_ia, 25.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label("Display boxes's weight: Yes")
            .set(widget_index.toggle_button_weight_boxes, ui)
            .last()
        {
        }
        
        let depth_range = ["1","2","3","4","5","6","7","8","9","10"];
        for selected_idx in widget::DropDownList::new(&depth_range, Some(0))
            .w_of(widget_index.homepage_canvas)
            .h(75.0)
            .middle_of(widget_index.homepage_canvas)
            .down_from(widget_index.toggle_button_weight_boxes, 25.0)
            .max_visible_items(5)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .scrollbar_on_top()
            .set(widget_index.dropdown_button_deph, ui)
        {
        }
    }
}