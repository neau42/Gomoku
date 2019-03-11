//! Game controller.

use crate::WidgetIds;
use crate::traits::view_controller::*;
use crate::traits::view_model::GameViewModel;
use crate::traits::player::*;
use crate::views::Game::GameView;
use crate::models::game::Game;
use crate::models::gameboard::*;
use crate::models::ia::IA;

use conrod::*;
use conrod::UiCell;
use conrod::widget::id::Id;
use std::collections::HashMap;

pub enum GameEvent {
	Grid()
}

pub struct GameController {
	pub view: GameView,
	event: fn(&Gameboard, &mut Box<Player>, Option<(usize, usize)>, Stone),
}



impl GameViewController for GameController {
	fn new(widget_ids: &WidgetIds) -> Box<GameController> {
		let view = GameView::new();
		let event = |state: &Gameboard, player: &mut Box<Player>, selected_move: Option<(usize, usize)>, stone: Stone| {
			let (x, y) = selected_move.unwrap();
			player.set_move(state.set_stone_on_cell(x, y, stone));
		};
		let controller = GameController {
			view,
			event,
		};
		Box::new(controller)
	}

	fn show(&self, model: &mut Box<dyn GameViewModel>, ui: &mut UiCell, widget_ids: &WidgetIds) {
		let model: &mut Game = model.get_model().downcast_mut::<Game>().unwrap();
		let stone = model.current_stone;
		let mut is_human = true;

		let current_player: &mut Box<Player> = model.get_current_player();
		if current_player.get_type() == PlayerType::Ia { 
			// current_player.downcast_mut::<IA>().unwrap();
			// let (_, selected_move) = model.mdtf(0, 2);
			is_human = false;
		}
		let current_move = current_player.get_move();
		current_player.set_move(None);
		drop(current_player);
		if current_move.is_some() {
			model.state = current_move.unwrap();
			model.current_stone.switch();
		}
		self.view.display_grid(ui, widget_ids, self.event, model, stone, is_human);
	}

    fn get_type(&self) -> PageType {
        PageType::Game
    }
}
