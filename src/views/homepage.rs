use crate::controllers::homepage::HomepageController;
use crate::controllers::homepage::HomepageEvent;
use crate::models::game_info::*;
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


    pub fn display_button_start(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &HomepageEvent, model: &mut GameInfo) {
        if widget::Button::new()
            .w_of(widget_ids.homepage_canvas)
            .h(75.0)
            .middle_of(widget_ids.homepage_canvas)
            .down_from(widget_ids.title, 50.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label("Start")
            .set(widget_ids.button_start, ui)
            .was_clicked()
        {
            if let HomepageEvent::button_start(event) = event {
                event(model);
            }
        }
    }

    pub fn display_dropdown_button_game_mode(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &HomepageEvent, model: &mut GameInfo) {
        let game_modes: [&str; 3] = [GameMode::PlayerVsPlayer.into(), GameMode::PlayerVsIa.into(), GameMode::IaVsIa.into()];

        for mode_index in widget::DropDownList::new(&game_modes, model.mode_index)
            .w_of(widget_ids.homepage_canvas)
            .h(75.0)
            .middle_of(widget_ids.homepage_canvas)
            .down_from(widget_ids.button_start, 25.0)
            .max_visible_items(3)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .scrollbar_on_top()
            .set(widget_ids.dropdown_button_game_mode, ui)
        {
            if let HomepageEvent::dropdown_button_game_mode(event) = event {
                event(model, Some(mode_index));
            }
        }
    }

    pub fn display_toggle_button(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &HomepageEvent, model: &mut GameInfo) {
        let label = format!("Display boxes's weight: {}", if model.display_weight {"yes"} else {"no"});
        if let Some(value) = widget::Toggle::new(model.display_weight)
            .w_of(widget_ids.homepage_canvas)
            .h(75.0)
            .middle_of(widget_ids.homepage_canvas)
            .down_from(widget_ids.dropdown_button_game_mode, 25.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label(&label[..])
            .set(widget_ids.toggle_button_weight_boxes, ui)
            .last()
        {
            if let HomepageEvent::toggle_button_weight_boxes(event) = event {
                event(model, value);
            }
        }
    }

    pub fn display_number_dialer_first_ia_depth(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &HomepageEvent, model: &mut GameInfo) {
        for new_depth in widget::NumberDialer::new(model.first_ia_depth, model.min_depth, model.max_depth, 0)
            .w_of(widget_ids.homepage_canvas)
            .h(75.0)
            .middle_of(widget_ids.homepage_canvas)
            .down_from(widget_ids.toggle_button_weight_boxes, 25.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label("First Ia depth: ")
            .set(widget_ids.number_dialer_first_ia_depth, ui)
        {
            println!("first ia");
            if let HomepageEvent::number_dialer_first_ia_depth(event) = event {
                event(model, new_depth);
            }
        }
    }

    pub fn display_number_dialer_second_ia_depth(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &HomepageEvent, model: &mut GameInfo) {
        for new_depth in widget::NumberDialer::new(model.second_ia_depth, model.min_depth, model.max_depth, 0)
            .w_of(widget_ids.homepage_canvas)
            .h(75.0)
            .middle_of(widget_ids.homepage_canvas)
            .down_from(widget_ids.number_dialer_first_ia_depth, 25.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label("Second Ia depth: ")
            .set(widget_ids.number_dialer_second_ia_depth, ui)
        {
            println!("second ia");
            if let HomepageEvent::number_dialer_second_ia_depth(event) = event {
                event(model, new_depth);
            }
        }
    }
}