//! Gameboard controller.

use conrod::UiCell;

use crate::WidgetIds;
use crate::traits::view_controller::*;
use crate::traits::view_model::GameViewModel;

use conrod::*;

use crate::views::gameboard::GameboardView;
use crate::models::gameboard::Gameboard;
use crate::models::gameboard::Stone;


pub struct GameboardController {
	pub view: GameboardView,
}

impl GameViewController for GameboardController {
	fn new(_widget_ids: &WidgetIds) -> Box<GameboardController> {
		let view = GameboardView::new();
		println!("new GameboardController!");
		let controller = GameboardController {
			view,
			};
		Box::new(controller)
	}

	fn show(&self, model: &mut Box<dyn GameViewModel>, ui: &mut UiCell, widget_ids: &WidgetIds) {
		let model: &mut Gameboard = match model.get_model().downcast_mut::<Gameboard>() {
			Some(model) => model,
			None => panic!("&GameViewModel isn't a Gameboard!"),
		};
		
		let color: Color;
		let stone: Stone;

		if model.test_switch == true {
			color = color::WHITE;
			stone = Stone::WHITE;
		} else {
			color = color::BLACK;
			stone = Stone::BLACK;
		}

		match self.view.display_grid(model, ui, widget_ids, color) {
			Some((x, y)) => {
				if model.set_stone_on_cell(x, y, stone){
					model.test_switch = !model.test_switch;
					}
				}
			_ => (),
		}
	}

    fn get_type(&self) -> PageType {
        PageType::Gameboard
    }
}
