//! Game controller.

use crate::WidgetIds;
use crate::traits::view_controller::*;
use crate::traits::view_model::GameViewModel;
use crate::traits::player::*;
use crate::views::Game::GameView;
use crate::models::game::Game;
use crate::models::gameboard::*;

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
			let (y, x) = selected_move.unwrap();
			player.set_move(state.set_stone_on_cell(y, x, stone));
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

		let stone = model.current_stone.clone();
		if let Some(new_state) = match model.get_current_player().get_type() {
			PlayerType::Human => {
				self.view.display_grid(ui, widget_ids, self.event, model, stone, true);
				model.get_current_player().get_move()
				// None//TMP
			},
			_ => {
				self.view.display_grid(ui, widget_ids, self.event, model, stone, false);
				// println!("je passe");
				let (_, selected_move) = model.mdtf(0, 2);
				// println!("j'ai fini");
				// dbg!(&selected_move);
				selected_move
			},
		} {
			model.state = new_state;
			model.current_stone = match model.current_stone {
				Stone::BLACK => Stone::WHITE,
				_ => Stone::BLACK,
			}
		}
		model.get_current_player().set_move(None);
	}

    fn get_type(&self) -> PageType {
        PageType::Game
    }
}
