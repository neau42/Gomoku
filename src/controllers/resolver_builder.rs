

use crate::WidgetIds;
use crate::models::resolver_builder::*;
use crate::views::resolver_builder::ResolverBuilderView;
use crate::traits::view_controller::*;
use crate::traits::view_model::GameViewModel;
use conrod::UiCell;
use std::collections::HashMap;
use conrod::widget::id::Id;


pub enum ResolverBuilderEvent {
    DropdownButtonGameMode(fn(&mut GameInfo, Option<usize>)),
    ToggleButtonWeightBoxes(fn(&mut GameInfo, bool)),
    NumberDialerFirstIaDepth(fn(&mut GameInfo, f32)),
    NumberDialerSecondIaDepth(fn(&mut GameInfo, f32)),
    ButtonStart(fn(model: &mut GameInfo)),
}

pub struct ResolverBuilderController {
    pub view: ResolverBuilderView,
    events: HashMap<Id, ResolverBuilderEvent>,
}

impl ResolverBuilderController {
    fn set_events(&mut self, widget_ids: &WidgetIds) {
        self.events.insert(widget_ids.dropdown_button_game_mode, ResolverBuilderEvent::DropdownButtonGameMode(|model: &mut GameInfo, mode_index: Option<usize>| {
            model.set_mode(mode_index) ;
        }));

        self.events.insert(widget_ids.toggle_button_weight_boxes, ResolverBuilderEvent::ToggleButtonWeightBoxes(|model: &mut GameInfo, display_weight: bool| {
            model.display_weight(display_weight);
        }));

        self.events.insert(widget_ids.number_dialer_first_ia_depth, ResolverBuilderEvent::NumberDialerFirstIaDepth(|model: &mut GameInfo, depth: f32| {
            model.set_first_ia_depth(depth);
        }));

        self.events.insert(widget_ids.number_dialer_second_ia_depth, ResolverBuilderEvent::NumberDialerSecondIaDepth(|model: &mut GameInfo, depth: f32| {
            model.set_second_ia_depth(depth);
        }));

        self.events.insert(widget_ids.button_start, ResolverBuilderEvent::ButtonStart(|model: &mut GameInfo| {
            model.change_window();
        }));
    }
}

impl GameViewController for ResolverBuilderController {
    fn new(widget_ids: &WidgetIds) -> Box<ResolverBuilderController> {
        let view = ResolverBuilderView::new();
        let mut controller = ResolverBuilderController {
            view,
            events: HashMap::new(),
        };
        controller.set_events(widget_ids);
        Box::new(controller)
    }
    
    fn show(&self, model:  &mut Box<dyn GameViewModel>, ui: &mut UiCell, widget_ids: &WidgetIds) {
        let model: &mut GameInfo = match model.get_model().downcast_mut::<GameInfo>() {
            Some(model) => model,
            None => panic!("&GameViewModel isn't a GameInfo!"),
        };
        self.view.display_canvas(ui, widget_ids);
        self.view.display_button_start(ui, widget_ids, self.events.get(&widget_ids.button_start).unwrap(), model);
        self.view.display_dropdown_button_game_mode(ui, widget_ids, self.events.get(&widget_ids.dropdown_button_game_mode).unwrap(), model); 
        self.view.display_toggle_button(ui, widget_ids, self.events.get(&widget_ids.toggle_button_weight_boxes).unwrap(), model);
        if model.mode_index != Some(0) {
            self.view.display_number_dialer_first_ia_depth(ui, widget_ids, self.events.get(&widget_ids.number_dialer_first_ia_depth).unwrap(), model);
        }
        if model.mode_index == Some(2) {
            self.view.display_number_dialer_second_ia_depth(ui, widget_ids, self.events.get(&widget_ids.number_dialer_second_ia_depth).unwrap(), model);
        }
    }

	// fn check_event(&mut self, _event: &Event, _model: &mut Box<dyn GameViewModel>, _ui: &mut UiCell, _widget_ids: &WidgetIds) {
	// 	()
	// }
    
    fn get_type(&self) -> PageType {
        PageType::ResolverBuilder
    }
}