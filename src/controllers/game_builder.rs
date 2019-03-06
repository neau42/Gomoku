

use crate::WidgetIds;
use crate::models::game_builder::*;
use crate::views::game_builder::GameBuilderView;
use crate::traits::view_controller::*;
use crate::traits::view_model::GameViewModel;
use conrod::UiCell;
use std::collections::HashMap;
use conrod::widget::id::Id;


pub enum GameBuilderEvent {
    DropdownButtonGameMode(fn(&mut GameBuilder, usize)),
    ToggleButtonWeightBoxes(fn(&mut GameBuilder, bool)),
    NumberDialerFirstIaDepth(fn(&mut GameBuilder, f32)),
    NumberDialerSecondIaDepth(fn(&mut GameBuilder, f32)),
    ButtonStart(fn(model: &mut GameBuilder)),
}

pub struct GameBuilderController {
    pub view: GameBuilderView,
    events: HashMap<Id, GameBuilderEvent>,
}

impl GameBuilderController {
    fn set_events(&mut self, widget_ids: &WidgetIds) {
        self.events.insert(widget_ids.dropdown_button_game_mode, GameBuilderEvent::DropdownButtonGameMode(|model: &mut GameBuilder, mode_index: usize| {
            model.set_mode(mode_index) ;
        }));

        self.events.insert(widget_ids.toggle_button_weight_boxes, GameBuilderEvent::ToggleButtonWeightBoxes(|model: &mut GameBuilder, display_weight: bool| {
            model.display_weight(display_weight);
        }));

        self.events.insert(widget_ids.number_dialer_first_ia_depth, GameBuilderEvent::NumberDialerFirstIaDepth(|model: &mut GameBuilder, depth: f32| {
            model.set_first_ia_depth(depth);
        }));

        self.events.insert(widget_ids.number_dialer_second_ia_depth, GameBuilderEvent::NumberDialerSecondIaDepth(|model: &mut GameBuilder, depth: f32| {
            model.set_second_ia_depth(depth);
        }));

        self.events.insert(widget_ids.button_start, GameBuilderEvent::ButtonStart(|model: &mut GameBuilder| {
            model.change_window();
        }));
    }
}

impl GameViewController for GameBuilderController {
    fn new(widget_ids: &WidgetIds) -> Box<GameBuilderController> {
        let view = GameBuilderView::new();
        let mut controller = GameBuilderController {
            view,
            events: HashMap::new(),
        };
        controller.set_events(widget_ids);
        Box::new(controller)
    }
    
    fn show(&self, model:  &mut Box<dyn GameViewModel>, ui: &mut UiCell, widget_ids: &WidgetIds) {
        let model: &mut GameBuilder = match model.get_model().downcast_mut::<GameBuilder>() {
            Some(model) => model,
            None => panic!("&GameViewModel isn't a GameBuilder!"),
        };
        self.view.display_canvas(ui, widget_ids);
        self.view.display_button_start(ui, widget_ids, self.events.get(&widget_ids.button_start).unwrap(), model);
        self.view.display_dropdown_button_game_mode(ui, widget_ids, self.events.get(&widget_ids.dropdown_button_game_mode).unwrap(), model); 
        self.view.display_toggle_button(ui, widget_ids, self.events.get(&widget_ids.toggle_button_weight_boxes).unwrap(), model);
        if model.mode_index != 0 {
            self.view.display_number_dialer_first_ia_depth(ui, widget_ids, self.events.get(&widget_ids.number_dialer_first_ia_depth).unwrap(), model);
        }
        if model.mode_index == 2 {
            self.view.display_number_dialer_second_ia_depth(ui, widget_ids, self.events.get(&widget_ids.number_dialer_second_ia_depth).unwrap(), model);
        }
    }

    fn get_type(&self) -> PageType {
        PageType::GameBuilder
    }
}