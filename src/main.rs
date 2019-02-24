use piston_window::*;
use gomoku::gui::window::*;
use gomoku::gui::color::*;
use gomoku::gui::buttons::Button;
use opengl_graphics::{GlGraphics, GlyphCache};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() {
	let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = window_new(WIDTH, HEIGHT, opengl);

	//Set Text Style
	let texture_settings = TextureSettings::new().filter(Filter::Nearest);
	let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
		.expect("Could not load font");

	let mut gl = GlGraphics::new(opengl);
	let mut events = Events::new(EventSettings::new().lazy(true));

	//Update window only when receiving input . Event are disable
	window.set_lazy(true);

	let button: Button = Button::new("My buttonMNFFD", fill_color(0,255,00, 1.0)).border(fill_color(255,0,0, 1.0), 5.0);

	//Boucle de jeu
	while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
			let draw_closure = |context: Context, graphic: &mut GlGraphics| {
				clear(fill_color(195, 155, 95, 1.0), graphic);
				graphic.clear_stencil(0);
				button.draw(&context, graphic);
			};
            gl.draw(args.viewport(), draw_closure);
		}
	}
}