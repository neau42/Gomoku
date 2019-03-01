extern crate piston;
extern crate graphics;

use piston_window::*;
use gomoku::gui::window::*;
use gomoku::gui::color::*;
use piston::event_loop::{Events, EventLoop, EventSettings};

use opengl_graphics::{GlGraphics};

mod views;
use views::gameboard::*;


mod controllers;
use controllers::gameboard::*;

mod models;
use models::gameboard::*;

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
        button_player_vs_player,
        button_player_vs_ia,
        toggle_button_weight_boxes,
        dropdown_button_deph,
        text,
        button_return_to_menu,
    }
}

fn main() {
	let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = window_new(WIDTH, HEIGHT, opengl);

    let mut ui = UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
	let widget_ids = WidgetIds::new(ui.widget_id_generator());

	let mut gl = GlGraphics::new(opengl);
	let mut events = Events::new(EventSettings::new().lazy(true));

	let gameboard = Gameboard::new();
	let gameboard_view = GameboardView::new();
	let mut gameboard_controller = GameboardController::new(gameboard, gameboard_view, &widget_ids);

	window.set_lazy(true);

	while let Some(e) = events.next(&mut window) {
		gameboard_controller.event(&e);
        if let Some(args) = e.render_args() {
			let draw_closure = |context: Context, graphic: &mut GlGraphics| {
				clear(fill_color(195, 155, 95, 1.0), graphic);
				graphic.clear_stencil(0);
				gameboard_controller.show(&context, graphic);
			};
            gl.draw(args.viewport(), draw_closure);


		}
	}
}