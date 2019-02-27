use crate::controllers::homepage::HomepageController;
use crate::controllers::homepage::Event;
use crate::models::game_info::GameMode;
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

    pub fn display_dropdown_button_game_mode(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &Event, controller: &mut HomepageController, mode: GameMode) {
        let depth_range = ["1","2","3","4","5","6","7","8","9","10"];
        for selected_idx in widget::DropDownList::new(&depth_range, Some(0))
            .w_of(widget_ids.homepage_canvas)
            .h(75.0)
            .middle_of(widget_ids.homepage_canvas)
            .down_from(widget_ids.title, 50.0)
            .max_visible_items(3)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .scrollbar_on_top()
            .set(widget_ids.dropdown_button_game_mode, ui)
        {
            if let Event::dropdown_button_game_mode(event) = event {
                event(controller, mode);//To change
            }
        }
    }

    pub fn display_toggle_button(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &Event, controller: &mut HomepageController, display_weight: bool) {
        if let Some(value) = widget::Toggle::new(true)
            .w_of(widget_ids.homepage_canvas)
            .h(75.0)
            .middle_of(widget_ids.homepage_canvas)
            .down_from(widget_ids.dropdown_button_game_mode, 25.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label("Display boxes's weight: Yes")
            .set(widget_ids.toggle_button_weight_boxes, ui)
            .last()
        {
            if let Event::toggle_button_weight_boxes(event) = event {
                event(controller, display_weight);//To change
            }
        }
    }

    pub fn display_dropdown_button_first_ia_deph(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &Event, controller: &mut HomepageController, depth: u8) {
        let depth_range = ["1","2","3","4","5","6","7","8","9","10"];
        for selected_idx in widget::DropDownList::new(&depth_range, Some(0))
            .w_of(widget_ids.homepage_canvas)
            .h(75.0)
            .middle_of(widget_ids.homepage_canvas)
            .down_from(widget_ids.toggle_button_weight_boxes, 25.0)
            .max_visible_items(3)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .scrollbar_on_top()
            .set(widget_ids.dropdown_button_first_ia_deph, ui)
        {
            if let Event::dropdown_button_first_ia_deph(event) = event {
                event(controller, depth);//To change
            }
        }
    }

    pub fn display_dropdown_button_second_ia_deph(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &Event, controller: &mut HomepageController, depth: u8) {
        let depth_range = ["1","2","3","4","5","6","7","8","9","10"];
        for selected_idx in widget::DropDownList::new(&depth_range, Some(0))
            .w_of(widget_ids.homepage_canvas)
            .h(75.0)
            .middle_of(widget_ids.homepage_canvas)
            .down_from(widget_ids.dropdown_button_first_ia_deph, 25.0)
            .max_visible_items(3)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .scrollbar_on_top()
            .set(widget_ids.dropdown_button_second_ia_deph, ui)
        {
            if let Event::dropdown_button_second_ia_deph(event) = event {
                event(controller, depth);//To change
            }
        }
    }

    pub fn display_button_start(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &Event, controller: &mut HomepageController) {
        if widget::Button::new()
            .w_of(widget_ids.homepage_canvas)
            .h(75.0)
            .middle_of(widget_ids.homepage_canvas)
            .down_from(widget_ids.homepage_canvas, 50.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label("Start")
            .set(widget_ids.dropdown_button_game_mode, ui)
            .was_clicked()
        {
            if let Event::button_start(event) = event {
                event(controller);
            }
        }
    }
}