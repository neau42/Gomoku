use graphics::{ Context, Graphics };
use graphics::types::Radius;
use graphics::types::Color;
use graphics::color;
use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::rectangle::Border;
use piston_window::*;

pub struct Button {
    label: String,
    text: Text,
    text_style: String,
    core: Rectangle,
    pos_x: f64,
    pos_y: f64,
    width: f64,
    height: f64,
}

impl Button {
    pub fn new(label: &str, color: Color) -> Button {
        Button {
            label: label.to_string(),
            text: Text::new_color(color::BLACK, 15),
            text_style: "assets/roboto.ttf".to_string(),
            core: Rectangle::new(color),
            pos_x: 50.0,
            pos_y: 50.0,
            width: 200.0,
            height: 100.0,
        }
    }

    pub fn title_color(mut self, color: Color) -> Self {
        self.text = Text::new_color(color, self.text.font_size);
        self
    }

    pub fn border(mut self, color: Color, radius: Radius) -> Self {
        self.core = self.core.border(Border {color, radius});
        self
    }

    pub fn draw(&self, context: &Context, graphic: &mut GlGraphics) {
        let rectangle = [self.pos_x, self.pos_y, self.width, self.height];
        self.core.draw(rectangle, &context.draw_state, context.transform, graphic);

        // Set Text Style
    	let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        dbg!(context.draw_state);
        dbg!(context.transform);
        dbg!(context.viewport.unwrap());
        dbg!(context.view);
        let transform = context.transform.trans(150.0, 100.0);

	    let ref mut glyphs = GlyphCache::new("assets/roboto.ttf", (), texture_settings).expect("Could not load font");
        self.text.draw("plop" , glyphs, &context.draw_state, transform, graphic).unwrap();

    }
}

pub struct ToggleButton {
    title: String,
    color: Color,
}

impl ToggleButton {
    pub fn new(title: String, color: Color) -> ToggleButton {
        ToggleButton {
            title,
            color,
        }
    }
}