use crate::controllers::homepage::HomepageController;
use crate::controllers::window::WindowController;

use std::io;
use std::io::{Error, ErrorKind};

pub struct gameplayController {
    window: WindowController,
    page: HomepageController,
}

impl gameplayController {
    pub fn new() -> gameplayController {
        let window = WindowController::new();
        let page = HomepageController::new();
        gameplayController {
            window,
            page,
        }
    }

    pub fn run_loop(&self) {
        let mut events = event_loop::EventLoop::new();
        'render: loop {
            for event in events.next(&mut events_loop) {
                if let Some(event) = convert_event(event.clone(), &display) {
                    ui.handle_event(event);
                    events.needs_update();
                }
                if controller.event(&event) {
                    break 'render;
                }
            }

            let ui = &mut ui.set_widgets();
            window_display(background, ui, &widget_ids);
            controller.show(ui, &widget_ids);
            
            // Draw the `Ui` if it has changed.
            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }

    pub fn init(&self) -> Result<_, io::Error> {
        //  cHeck file exists
    }
}

// const WIDTH: u32 = 1600;
// const HEIGHT: u32 = 1024;



// mod event_loop;
// mod models;
// mod views;


// use models::homepage::*;
// use views::homepage::*;
// use controllers::homepage::*;
// use conrod::backend::glium::glium::glutin::*;
// use conrod::backend::glium::glium::Display;
// use conrod::backend::winit::convert_event;
// use conrod::color;
// use ::image::open;
// use conrod::*;
// use conrod::glium::Surface;
// use std::path::Path;

// widget_ids! {
//     pub struct WidgetIds {
//         background,
//         title,
//         window_canvas,
//         window_canvas_y_scrollbar,
//         window_canvas_x_scrollbar,
//         homepage_canvas,
//         button_player_vs_player,
//         button_player_vs_ia,
//         toggle_button_weight_boxes,
//         dropdown_button_deph,
//         text,
//     }
// }

// fn load_background(display: &glium::Display) -> glium::texture::Texture2d {
//     let rgba_image = open(&Path::new("assets/images/wood.jpg")).unwrap().to_rgba();
//     let image_dimensions = rgba_image.dimensions();
//     let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), image_dimensions);
//     let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
//     texture
// }

// fn window_display(background: conrod::image::Id, ui: &mut UiCell, widget_ids: &WidgetIds) {
//     widget::Image::new(background).wh_of(ui.window).middle_of(ui.window).set(widget_ids.background, ui);
//     widget::Canvas::new()
//         .border(1.0)
//         .pad(50.0)
//         .color(color::TRANSPARENT)
//         .scroll_kids()
//         .set(widget_ids.window_canvas, ui);
//     widget::Scrollbar::x_axis(widget_ids.window_canvas).auto_hide(true).set(widget_ids.window_canvas_y_scrollbar, ui);
//     widget::Scrollbar::y_axis(widget_ids.window_canvas).auto_hide(true).set(widget_ids.window_canvas_x_scrollbar, ui);
//     widget::Text::new("Gomoku")
//         .mid_top_of(widget_ids.window_canvas)
//         .font_size(50)
//         .color(color::BLACK)
//         .set(widget_ids.title, ui);
// }














    // let mut events_loop = EventsLoop::new();
    // let window_builder = WindowBuilder::new()
    //     .with_decorations(false)
    //     .with_dimensions((WIDTH, HEIGHT).into());
    // let context_builder = ContextBuilder::new()
    //     .with_vsync(true)
    //     .with_multisampling(4);
    // let display = Display::new(window_builder, context_builder, &events_loop).unwrap();
    // let mut ui = UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    // let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    // let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    // ui.fonts.insert_from_file(Path::new("assets/fonts/FiraSans-Regular.ttf")).unwrap();
    
    // let background = image_map.insert(load_background(&display));
    // let widget_ids = WidgetIds::new(ui.widget_id_generator());

    // let model = HomepageModel::new();
    // let view = HomepageView::new();
    // let controller = HomepageController::new(view, model, &widget_ids);

    //renderLoop