//! Game view.
use crate::controllers::game::GameEvent;
use crate::traits::player::Player;
use crate::models::gameboard::Gameboard;
use crate::widgets::gameboard as CustomWidget;
use crate::WidgetIds;
use crate::models::gameboard::Stone;
use crate::models::game::Game;
use conrod::*;

pub struct GameView {
    pub position: [f64; 2],
    pub size: f64,
	pub hoshi_size: f64,
}

impl GameView {
	pub fn new() -> GameView {
		GameView {
			position: [36.0; 2],
			size: 800.0,
			hoshi_size: 15.0,
		}
	}

	pub fn display_grid(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: fn(&Gameboard, &mut Box<Player>, Option<(usize, usize)>, Stone), model: &mut Game, stone: Stone, is_human: bool) {
		if let Some((x, y)) = CustomWidget::Board::new(&model.state, stone, is_human)
			.middle_of(widget_ids.window_canvas)
			.down_from(widget_ids.title, 15.0)
			.w_h(self.size, self.size)
			.set(widget_ids.grid, ui)
			.was_clicked() {
				event(&model.state.clone(), model.get_current_player(), Some((x, y)), stone);
			}
	}
}