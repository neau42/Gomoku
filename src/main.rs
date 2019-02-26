extern crate piston;
extern crate graphics;

use piston_window::*;
use gomoku::gui::window::*;
use gomoku::gui::color::*;
use piston::event_loop::{Events, EventLoop, EventSettings};

use opengl_graphics::{GlGraphics, GlyphCache};

use crate::gameboard::Gameboard;

mod gameboard;

mod views;
use views::gameboard::*;


mod controllers;
use controllers::gameboard::*;

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 1024;

fn main() {
	let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = window_new(WIDTH, HEIGHT, opengl);

	//Set Text Style
	let texture_settings = TextureSettings::new().filter(Filter::Nearest);
	let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
		.expect("Could not load font");;

	let mut gl = GlGraphics::new(opengl);
	let mut events = Events::new(EventSettings::new().lazy(true));

	let gameboard = Gameboard::new();
	let mut gameboard = GameboardController::new(gameboard);
	let gameboard_view_settings = GameboardViewSettings::new();
	let gameboard_view = GameboardView::new(gameboard_view_settings);

	//Update window only when receiving input . Event are disable
	window.set_lazy(true);
	//Boucle de jeu

	while let Some(e) = events.next(&mut window) {
		gameboard.event(gameboard_view.settings.size, &e);
        if let Some(args) = e.render_args() {
			let draw_closure = |context: Context, graphic: &mut GlGraphics| {
				clear(fill_color(195, 155, 95, 1.0), graphic);
				graphic.clear_stencil(0);
				gameboard_view.draw(&gameboard, glyphs, &context, graphic);
			};
            gl.draw(args.viewport(), draw_closure);


		}
	}
}