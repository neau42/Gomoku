//! Game controller.

use crate::WidgetIds;
use crate::traits::view_controller::*;
use crate::traits::view_model::GameViewModel;
use crate::views::game::GameView;
use crate::models::gameboard::*;
use crate::models::game::*;

use conrod::UiCell;
use conrod::widget::id::Id;
use std::collections::HashMap;

use crate::minmax_alphabeta;

pub enum GameEvent {
	Grid(fn(&mut Game, usize, usize)),
	ButtonUndo(fn(&mut Game)),
	ButtonQuit(fn(&mut Game)),
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
				model.update_last_move_time();
			}
        }));

		self.events.insert(widget_ids.button_undo, GameEvent::ButtonUndo(|model: &mut Game| {
			match model.game_mode {
				GameMode::PlayerVsPlayer => {
					if model.all_state.len() > 1 {
						model.all_state.pop();
						model.state = model.all_state.last().unwrap().clone();
						model.current_stone.switch();
					}
				},
				_ => {
					if model.all_state.len() > 2 {
						model.all_state.pop();
						model.all_state.pop();
						model.state = model.all_state.last().unwrap().clone();
					}
				}
			}
		}));

		self.events.insert(widget_ids.button_quit, GameEvent::ButtonQuit(|model: &mut Game| {
			model.change_window();
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
		
		let mut is_human = true;
		if let Player::Ia{ia, ..} = match model.current_stone {
			Stone::WHITE => &model.white_player,
			_ => &model.black_player,
		}
		{
			let best_move = if model.all_state.len() == 1 {
				let position = model.state.size / 2;
				Some((position, position))
			}
			else {
				ia.negascout(&mut model.state, &model.current_stone, ia.depth, isize::from(std::i16::MIN), isize::from(std::i16::MAX)).1
			};
			// // None possible ?
			// println!("depth = {}", ia.depth);
			match best_move{
				Some(best_move) => {
					if model.state.make_move(best_move.0, best_move.1, model.current_stone) {
						model.all_state.push(model.state.clone());
						model.current_stone.switch();
						model.update_last_move_time();
					}
				}
				None => println!("banana"),
			};
			is_human = false;
		}
		self.view.display_grid(ui, widget_ids, self.events.get(&widget_ids.grid).unwrap(), model, is_human);
		self.view.display_player_turn(ui, widget_ids, model);
		self.view.display_captures(ui, widget_ids, model.black_player.captures(), model.white_player.captures());
		self.view.display_last_move_time(ui, widget_ids, &model.last_move_time[..]);
		self.view.display_button_quit(ui, widget_ids, self.events.get(&widget_ids.button_quit).unwrap(), model);
		if (model.game_mode != GameMode::IaVsIa) {
			self.view.display_button_undo(ui, widget_ids, self.events.get(&widget_ids.button_undo).unwrap(), model);
		}
		// if !is_human {
		// 	model.state = current_move.unwrap();
		// 	model.current_stone.switch();
		// }
	}

    fn get_type(&self) -> PageType {
        PageType::Game
    }
}




		// let model: &mut Game = match model.get_model().downcast_mut::<Game>() {
		// 	Some(model) => model,
		// 	None => panic!("&GameViewModel isn't a Game!"),
		// };

		// let stone = model.current_stone.clone();
		// if let Some(new_state) = match model.get_current_player().get_type() {
		// 	PlayerType::Human => {
		// 		self.view.display_grid(ui, widget_ids, self.event, model, stone, true);
		// 		model.get_current_player().get_move()
		// 	},
		// 	_ => {
		// 		self.view.display_grid(ui, widget_ids, self.event, model, stone, false);
		// 		let (_, selected_move) = minmax_alphabeta::algo(&mut model.state, model.current_stone);

		// 		let test = selected_move.clone();
		// 		match test {
		// 			Some(test) => {
		// 				println!("\nIA:");
		// 				test.printboard();
		// 				println!("\nValue: {}, Max {}", test.value, test.max);

		// 				// test.print_all_align();
		// 			},
		// 			None => (),
		// 		}
		// 		// let (_, selected_move) = model.mdtf(0, 1);
		// 		// println!("j'ai fini");
		// 		// dbg!(&selected_move);
		// 		selected_move
		// 	},
		// } {
		// 	model.state = new_state;
		// 	model.current_stone = match model.current_stone {
		// 		Stone::BLACK => Stone::WHITE,
		// 		_ => Stone::BLACK,
		// 	}