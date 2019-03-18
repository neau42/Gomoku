//! Game view.
use crate::controllers::game::GameEvent;
use crate::widgets::gameboard as CustomWidget;
use crate::WidgetIds;
use crate::models::game::Game;
use crate::models::gameboard::Stone;
use conrod::*;

pub struct GameView {
    pub position: [f64; 2],
    pub size: f64,
	pub hoshi_size: f64,
}

#[rustfmt::skip]
impl GameView {
	pub fn new() -> GameView {
		GameView {
			position: [36.0; 2],
			size: 800.0,
			hoshi_size: 15.0,
		}
	}

	pub fn display_grid(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &GameEvent, model: &mut Game, is_human: bool) {
		if let Some((x, y)) = CustomWidget::Board::new(&model.state, model.current_stone, is_human)
			.top_left_of(widget_ids.window_canvas)
			.down_from(widget_ids.title, 50.0)
			.w_h(self.size, self.size)
			.set(widget_ids.grid, ui)
			.was_clicked() {
				if let GameEvent::Grid(event) = event {
					event(model, x, y);
				}
			}
	}
	pub fn display_player_turn(&self, ui: &mut UiCell, widget_ids: &WidgetIds, model: &mut Game) {
		let text = match model.current_stone {
			Stone::BLACK => "Player turn : Black",
			_ => "Player turn : White"
		};
		widget::Text::new(text)
            .right_from(widget_ids.grid, 50.0)
            .font_size(25)
            .color(color::BLACK)
            .set(widget_ids.text_turn, ui);
	}

	pub fn display_captures(&self, ui: &mut UiCell, widget_ids: &WidgetIds, black_capture: u8, white_capture: u8) {
		let text = format!("Black player capture: {}\nWhite player capture: {}", black_capture, white_capture);
		widget::Text::new(&text[..])
            .right_from(widget_ids.grid, 50.0)
            .down_from(widget_ids.text_turn, 25.0)
            .font_size(25)
            .color(color::BLACK)
            .set(widget_ids.text_captures, ui);
	}

	pub fn display_last_move_time(&self, ui: &mut UiCell, widget_ids: &WidgetIds, last_move_time: &str) {
		widget::Text::new(last_move_time)
            .right_from(widget_ids.grid, 50.0)
            .down_from(widget_ids.text_captures, 25.0)
            .font_size(25)
            .color(color::BLACK)
            .set(widget_ids.text_last_move_time, ui);
	}

    pub fn display_result(&self, ui: &mut UiCell, widget_ids: &WidgetIds, result: &str) {
		widget::Text::new(result)
            .right_from(widget_ids.grid, 50.0)
            .down_from(widget_ids.text_last_move_time, 25.0)
            .font_size(25)
            .set(widget_ids.text_result, ui);
	}


	pub fn display_button_quit(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &GameEvent, model: &mut Game) {
        if widget::Button::new()
            .h(75.0)
			.w(150.0)
            .bottom_right_of(widget_ids.grid)
            .right_from(widget_ids.grid, 50.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label("Quit")
            .set(widget_ids.button_quit, ui)
            .was_clicked()
        {
            if let GameEvent::ButtonQuit(event) = event {
                event(model);
            }
        }
	}

	pub fn display_button_undo(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &GameEvent, model: &mut Game) {
        if widget::Button::new()
            .h(75.0)
			.w(150.0)
            .up_from(widget_ids.button_quit, 25.0)
            .right_from(widget_ids.grid, 50.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label("Undo")
            .set(widget_ids.button_undo, ui)
            .was_clicked()
        {
            if let GameEvent::ButtonUndo(event) = event {
                event(model);
            }
        }
	}
}