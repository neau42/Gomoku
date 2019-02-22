use piston_window::*;

// Construct the window.
pub fn window_new(width: u32, height: u32, opengl: OpenGL) -> PistonWindow {
	WindowSettings::new("", [width, height])
		.opengl(opengl)
		.decorated(false)
		.samples(4)
		.exit_on_esc(true)
		.vsync(true)
		.build()
		.unwrap()
}