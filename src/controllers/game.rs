//! Game controller.

use crate::WidgetIds;
use crate::traits::view_controller::*;
use crate::traits::view_model::GameViewModel;
use crate::traits::player::*;
use crate::views::Game::GameView;
use crate::models::game::Game;
use crate::models::gameboard::Stone;

use conrod::*;
use conrod::UiCell;
use conrod::widget::id::Id;
use std::collections::HashMap;

pub enum GameEvent {
	Grid()
}

pub struct GameController {
	pub view: GameView,
	event: fn(&mut Box<Player>, Option<(usize, usize)>),
}



impl GameViewController for GameController {
	fn new(widget_ids: &WidgetIds) -> Box<GameController> {
		let view = GameView::new();
		let event = |player: &mut Box<Player>, selected_move: Option<(usize, usize)>| {
			player.set_move(selected_move);
		};
		let controller = GameController {
			view,
			event,
		};
		Box::new(controller)
	}


	fn show(&self, model: &mut Box<dyn GameViewModel>, ui: &mut UiCell, widget_ids: &WidgetIds) {
		let model: &mut Game = match model.get_model().downcast_mut::<Game>() {
			Some(model) => model,
			None => panic!("&GameViewModel isn't a Game!"),
		};

		let (color, stone, player) = if model.is_black_turn {
			(color::WHITE, Stone::WHITE, &mut model.white_player)
		}
		else {
			(color::BLACK, Stone::BLACK, &mut model.black_player)
		};

		if player.get_type() == PlayerType::Human {
			self.view.display_grid(ui, widget_ids, self.event, &model.state, player, color);
		}
		else {
			//Call ia function
			//puis set
		}

		if let Some((y, x)) = player.get_move() {
			if model.state.set_stone_on_cell(y, x, stone) {
				model.is_black_turn = !model.is_black_turn;
			}
			player.set_move(None);
		}
	}

    fn get_type(&self) -> PageType {
        PageType::Game
    }
}
