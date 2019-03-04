//! Gameboard view.

use crate::models::gameboard::Gameboard;
use crate::views::widget_gameboard;
use crate::WidgetIds;
use conrod::*;

pub struct GameboardView {
    pub position: [f64; 2],
    pub size: f64,
	pub hoshi_size: f64,
	// pub test_switch: bool,
}

impl GameboardView {
	pub fn new() -> GameboardView {
		GameboardView {
			position: [36.0; 2],
			size: 800.0,
			hoshi_size: 15.0,
			// test_switch: true,

		}
	}

	pub fn display_grid(& self, model: & Gameboard, ui: &mut UiCell, widget_ids: &WidgetIds, color: Color) -> Option<(usize, usize)> {
	
	if let Some((y, x)) = widget_gameboard::Board::new(model, color)
		.middle_of(widget_ids.window_canvas)
		.down_from(widget_ids.title, 15.0)
		.w_h(self.size, self.size)
		.set(widget_ids.grid, ui)
		.was_clicked() {
			return Some((x, y));
		}
	None
	}
}