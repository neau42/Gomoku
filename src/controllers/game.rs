//! Game controller.

use conrod::UiCell;

use crate::WidgetIds;
use crate::traits::view_controller::*;
use crate::traits::view_model::GameViewModel;

use conrod::*;

use crate::views::Game::GameView;
use crate::models::Game::Game;
use crate::models::gameboard::Stone;

// pub enum GameEvent {
	// Grid(fn(&mut ))
// }

pub struct GameController {
	pub view: GameView,
}

impl GameViewController for GameController {
	fn new(_widget_ids: &WidgetIds) -> Box<GameController> {
		let view = GameView::new();
		let controller = GameController {
			view,
			};
		Box::new(controller)
	}

	fn show(&self, model: &mut Box<dyn GameViewModel>, ui: &mut UiCell, widget_ids: &WidgetIds) {
		let model: &mut Game = match model.get_model().downcast_mut::<Game>() {
			Some(model) => model,
			None => panic!("&GameViewModel isn't a Game!"),
		};
		let color: Color;
		let stone: Stone;

		if model.state.test_switch == true {
			color = color::WHITE;
			stone = Stone::WHITE;
		} else {
			color = color::BLACK;
			stone = Stone::BLACK;
		}

		match self.view.display_grid(model, ui, widget_ids, color) {
			Some((x, y)) => {
				if model.state.set_stone_on_cell(x, y, stone){
					model.state.test_switch = !model.state.test_switch;
					}
				}
			_ => (),
		}
	}

    fn get_type(&self) -> PageType {
        PageType::Game
    }
}
