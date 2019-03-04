//! Resolver controller.

use conrod::UiCell;

use crate::WidgetIds;
use crate::traits::view_controller::*;
use crate::traits::view_model::GameViewModel;

use conrod::*;

use crate::views::resolver::ResolverView;
use crate::models::resolver::Resolver;
use crate::models::gameboard::Stone;


pub struct ResolverController {
	pub view: ResolverView,
}

impl GameViewController for ResolverController {
	fn new(_widget_ids: &WidgetIds) -> Box<ResolverController> {
		let view = ResolverView::new();
		let controller = ResolverController {
			view,
			};
		Box::new(controller)
	}

	fn show(&self, model: &mut Box<dyn GameViewModel>, ui: &mut UiCell, widget_ids: &WidgetIds) {
		let model: &mut Resolver = match model.get_model().downcast_mut::<Resolver>() {
			Some(model) => model,
			None => panic!("&GameViewModel isn't a Resolver!"),
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
        PageType::Resolver
    }
}
