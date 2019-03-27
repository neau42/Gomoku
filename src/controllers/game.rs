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
use std::collections::HashSet;

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
				model.current_stone = opposite_stone!(model.current_stone);
				model.update_last_move_time();
			}
        }));

		self.events.insert(widget_ids.button_undo, GameEvent::ButtonUndo(|model: &mut Game| {
			match model.game_mode {
				GameMode::PlayerVsPlayer => {
					if model.all_state.len() > 1 {
						model.all_state.pop();
						model.state = model.all_state.last().unwrap().clone();
						model.current_stone = opposite_stone!(model.current_stone);
						model.state.result = None;
					}
				},
				_ => {
					if model.all_state.len() > 2 {
						model.all_state.pop();
						model.all_state.pop();
						model.state = model.all_state.last().unwrap().clone();
						model.state.result = None;
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

	fn show(&self, model: &mut dyn GameViewModel, ui: &mut UiCell, widget_ids: &WidgetIds) {
		let model: &mut Game = model.get_model().downcast_mut::<Game>().unwrap();
		
		let mut is_human = true;
		if let Player::Ia{ia, ..} = match model.current_stone {
			WHITE => &model.white_player,
			_ => &model.black_player,
		}
		{
			let best_move = if model.all_state.len() == 1 {
				let position = SIZE / 2;
				Some((position, position))
			}
			else {
				let mut transposition_table: HashSet<Gameboard> = HashSet::new();
				ia.negascout(&mut model.state, &mut transposition_table, model.current_stone, ia.depth, isize::from(std::i16::MIN), isize::from(std::i16::MAX));
				// ia.alphabeta(&mut model.state, &mut transposition_table, model.current_stone, ia.depth, isize::from(std::i16::MIN), isize::from(std::i16::MAX));
				model.state.selected_move
			};
			match best_move{
				Some(best_move) => {
					if model.state.make_move(best_move.0, best_move.1, model.current_stone) {
						model.all_state.push(model.state.clone());
						model.current_stone = opposite_stone!(model.current_stone);
						model.update_last_move_time();
					}
				}
				None => print_all_state(&model.all_state),//println!("banana"),
			};
			is_human = false;
		}
		self.view.display_grid(ui, widget_ids, &self.events[&widget_ids.grid], model, is_human);
		self.view.display_player_turn(ui, widget_ids, model);
		self.view.display_captures(ui, widget_ids, model.state.black_captures, model.state.white_captures);
		self.view.display_last_move_time(ui, widget_ids, &model.last_move_time[..]);
		if model.state.is_finish() {
			let result: &str = match model.state.result.unwrap() {
				GameResult::BlackWin => "BLACK PLAYER WIN!",
				GameResult::WhiteWin => "WHITE PLAYER WIN!",
				GameResult::Equality => "EQUALITY!",
			};
			self.view.display_result(ui, widget_ids, result);
		}
		self.view.display_button_quit(ui, widget_ids, &self.events[&widget_ids.button_quit], model);
		if model.game_mode != GameMode::IaVsIa {
			self.view.display_button_undo(ui, widget_ids, &self.events[&widget_ids.button_undo], model);
		}
	}

    fn get_type(&self) -> PageType {
        PageType::Game
    }
}

fn print_all_state(all_state: &Vec<Gameboard> ) {

	println!("ALL STATES: ");
	for state in all_state {
		printboard!(state.cells);
	}
}