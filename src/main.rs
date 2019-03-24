//! Gomoku!
//! 
#[macro_use] extern crate conrod_derive;
#[macro_use] mod macros;
mod models;
mod views;
mod controllers;
mod utils;
mod widgets;
mod traits;

extern crate rand;


use controllers::gameplay::*;
use conrod::*;

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 1024;

widget_ids! {
    pub struct WidgetIds {
        background,
        title,
        window_canvas,
        game_builder_canvas,
        dropdown_button_game_mode,
        toggle_button_weight_boxes,
        number_dialer_first_ia_depth,
        number_dialer_second_ia_depth,
        button_start,
		grid,
        text_turn,
        text_captures,
        text_last_move_time,
        text_result,
        button_quit,
        button_undo,
    }
}

fn main() {
    let mut ui = UiBuilder::new([f64::from(WIDTH), f64::from(HEIGHT)]).build();
    let widget_ids = WidgetIds::new(ui.widget_id_generator());

    let mut gameplay: GameplayController = GameplayController::new(WIDTH, HEIGHT, ui, widget_ids);

    gameplay.run()
}
