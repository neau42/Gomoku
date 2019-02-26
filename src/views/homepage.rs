use crate::WidgetIds;
use conrod::*;

pub struct HomepageView {
}

impl HomepageView {
    pub fn new() -> HomepageView {
        HomepageView {
        }
    }

    pub fn display_canvas(&self, ui: &mut UiCell, widget_ids: &WidgetIds) {
        widget::Canvas::new()
            .w(500.0)
            .border(0.0)
            .middle_of(widget_ids.window_canvas)
            .down_from(widget_ids.title, 50.0)
            .color(color::TRANSPARENT)
            .set(widget_ids.homepage_canvas, ui);
    }

    pub fn display_button_vs_player(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &fn()) {
        if widget::Button::new()
            .w_of(widget_ids.homepage_canvas)
            .h(75.0)
            .mid_top_of(widget_ids.homepage_canvas)
            .down_from(widget_ids.title, 50.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label("Player vs Player")
            .set(widget_ids.button_player_vs_player, ui)
            .was_clicked()
        {
            event();
        }
    }

    pub fn display_button_vs_ia(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &fn()) {
        if widget::Button::new()
            .w_of(widget_ids.homepage_canvas)
            .h(75.0)
            .middle_of(widget_ids.homepage_canvas)
            .down_from(widget_ids.button_player_vs_player, 25.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label("Player vs IA")
            .set(widget_ids.button_player_vs_ia, ui)
            .was_clicked()
        {
            event();
        }
    }

    pub fn display_toggle_button(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &fn()) {
        if let Some(value) = widget::Toggle::new(true)
            .w_of(widget_ids.homepage_canvas)
            .h(75.0)
            .middle_of(widget_ids.homepage_canvas)
            .down_from(widget_ids.button_player_vs_ia, 25.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label("Display boxes's weight: Yes")
            .set(widget_ids.toggle_button_weight_boxes, ui)
            .last()
        {
            event();
        }
    }

    pub fn display_dropdown_button_deph(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &fn()) {
        let depth_range = ["1","2","3","4","5","6","7","8","9","10"];
        for selected_idx in widget::DropDownList::new(&depth_range, Some(0))
            .w_of(widget_ids.homepage_canvas)
            .h(75.0)
            .middle_of(widget_ids.homepage_canvas)
            .down_from(widget_ids.toggle_button_weight_boxes, 25.0)
            .max_visible_items(5)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .scrollbar_on_top()
            .set(widget_ids.dropdown_button_deph, ui)
        {
            event();
        }
    }
}