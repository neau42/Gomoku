

use crate::WidgetIds;
use crate::models::game_info::*;
use crate::views::homepage::HomepageView;
use crate::traits::view_controller::*;
use conrod::UiCell;
use std::collections::HashMap;
use conrod::widget::id::Id;

pub enum Event {
    dropdown_button_game_mode(fn(&mut HomepageController, GameMode)),
    toggle_button_weight_boxes(fn(&mut HomepageController, bool)),
    dropdown_button_first_ia_deph(fn(&mut HomepageController, u8)),
    dropdown_button_second_ia_deph(fn(&mut HomepageController, u8)),
    button_start(fn(&mut HomepageController)),
}

pub struct HomepageController {
    pub view: HomepageView,
    pub model: GameInfo,
    events: HashMap<Id, Event>,
    change_window: bool,
}

impl HomepageController {
    fn set_events(&mut self, widget_ids: &WidgetIds) {
        self.events.insert(widget_ids.dropdown_button_game_mode, Event::dropdown_button_game_mode(|controller: &mut HomepageController, mode: GameMode| {
            controller.set_mode(mode)
        }));

        self.events.insert(widget_ids.toggle_button_weight_boxes, Event::toggle_button_weight_boxes(|controller: &mut HomepageController, display_weight: bool| {
            controller.display_weight(display_weight);
        }));

        self.events.insert(widget_ids.dropdown_button_first_ia_deph, Event::dropdown_button_first_ia_deph(|controller: &mut HomepageController, depth: u8| {
            controller.set_first_ia_depth(depth);
        }));

        self.events.insert(widget_ids.dropdown_button_second_ia_deph, Event::dropdown_button_second_ia_deph(|controller: &mut HomepageController, depth: u8| {
            controller.set_second_ia_depth(depth);
        }));

        self.events.insert(widget_ids.button_start, Event::button_start(|controller: &mut HomepageController| {
            controller.change_window = true;
        }));
    }
}

impl GameViewController for HomepageController {
    fn new(widget_ids: &WidgetIds) -> Box<HomepageController> {
        let model = GameInfo::new();
        let view = HomepageView::new();
        let mut controller = HomepageController {
            view,
            model,
            events: HashMap::new(),
            change_window: false,
        };
        controller.set_events(widget_ids);
        Box::new(controller)
    }
    
    fn show(&mut self, ui: &mut UiCell, widget_ids: &WidgetIds) {
        self.view.display_canvas(ui, widget_ids);
        self.view.display_dropdown_button_game_mode(ui, widget_ids, self.events.get(&widget_ids.dropdown_button_game_mode).unwrap(), self, self.model.mode);
        self.view.display_toggle_button(ui, widget_ids, self.events.get(&widget_ids.toggle_button_weight_boxes).unwrap(), self, self.model.display_weight);
        if (self.model.mode != GameMode::PlayerVsPlayer) {
            self.view.display_dropdown_button_first_ia_deph(ui, widget_ids, self.events.get(&widget_ids.dropdown_button_first_ia_deph).unwrap(), self, self.model.first_ia_depth);
        }
        if (self.model.mode == GameMode::IaVsIa) {
            self.view.display_dropdown_button_second_ia_deph(ui, widget_ids, self.events.get(&widget_ids.dropdown_button_second_ia_deph).unwrap(), self, self.model.second_ia_depth);
        }
        self.view.display_button_start(ui, widget_ids, self.events.get(&widget_ids.button_start).unwrap(), self);
    }
    
    fn get_type(&self) -> PageType {
        PageType::Homepage
    }
    
    fn need_change_window(&self) -> bool {
        self.change_window
    }
}

impl HomepageController {
    fn set_mode(&mut self, mode: GameMode) {
        self.model.mode = mode;
    }

    fn display_weight(&mut self, display_weight: bool) {
        self.model.display_weight = display_weight;
    }

    fn set_first_ia_depth(&mut self, depth: u8) {
        self.model.first_ia_depth = depth;
    }

    fn set_second_ia_depth(&mut self, depth: u8) {
        self.model.second_ia_depth = depth;
    }
}