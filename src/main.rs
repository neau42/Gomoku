mod controllers;
mod views;
mod models;
mod utils;
mod traits;

use controllers::gameplay::*;
use conrod::*;

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 1024;

widget_ids! {
    pub struct WidgetIds {
        background,
        title,
        window_canvas,
        window_canvas_y_scrollbar,
        window_canvas_x_scrollbar,
        homepage_canvas,
        dropdown_button_game_mode,
        toggle_button_weight_boxes,
        dropdown_button_first_ia_deph,
        dropdown_button_second_ia_deph,
        button_start,
        text,
    }
}

fn main() {
    let mut ui = UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    let widget_ids = WidgetIds::new(ui.widget_id_generator());

    let mut gameplay: GameplayController = GameplayController::new(WIDTH, HEIGHT, ui, widget_ids);
    
    gameplay.run()
}
