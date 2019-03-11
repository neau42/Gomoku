//! Game controller.

use crate::WidgetIds;
use crate::traits::view_controller::*;
use crate::traits::view_model::GameViewModel;
use crate::views::game::GameView;
use crate::models::game::*;

use conrod::UiCell;
use conrod::widget::id::Id;
use std::collections::HashMap;

pub enum GameEvent {
	Grid(fn(&mut Game, usize, usize)),
	ButtonUndo(fn(&mut Game)),
}

pub struct GameController {
	pub view: GameView,
	events: HashMap<Id, GameEvent>,
}

impl GameController {
	fn set_events(&mut self, widget_ids: &WidgetIds) {
		self.events.insert(widget_ids.grid, GameEvent::Grid(|model: &mut Game, x: usize, y: usize| {
			if model.state.make_move(x, y, model.current_stone) {
				model.all_state.push(model.state.clone());
				model.current_stone.switch();
			}
        }));

		self.events.insert(widget_ids.button_undo, GameEvent::ButtonUndo(|model: &mut Game| {
			if model.all_state.len() > 1 {
				model.all_state.pop();
				model.state = model.all_state.last().unwrap().clone();
				model.current_stone.switch();
			}
		}));
	}
}


impl GameViewController for GameController {
	fn new(widget_ids: &WidgetIds) -> Box<GameController> {
		let view = GameView::new();
		let mut controller = GameController {
			view,
			events: HashMap::new(),
		};
		controller.set_events(widget_ids);
		Box::new(controller)
	}

	fn show(&self, model: &mut Box<dyn GameViewModel>, ui: &mut UiCell, widget_ids: &WidgetIds) {
		let model: &mut Game = model.get_model().downcast_mut::<Game>().unwrap();
		
		let current_player: &mut Player = model.get_current_player();
		let mut is_human = true;
		if let Player::Ia{ia, ..} = current_player  {
			// ia.do_stuff
			is_human = false;
		}
		drop(current_player);
		self.view.display_grid(ui, widget_ids, self.events.get(&widget_ids.grid).unwrap(), model, is_human);
		self.view.display_button_undo(ui, widget_ids, self.events.get(&widget_ids.button_undo).unwrap(), model);
		// if !is_human {
		// 	model.state = current_move.unwrap();
		// 	model.current_stone.switch();
		// }
	}

    fn get_type(&self) -> PageType {
        PageType::Game
    }
}
