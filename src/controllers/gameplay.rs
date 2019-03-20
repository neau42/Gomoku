use crate::controllers::game::GameController;
use crate::controllers::game_builder::GameBuilderController;
use crate::controllers::window::WindowController;
use crate::models::game_builder::*;
use crate::models::game::Game;
use crate::traits::view_controller::GameViewController;
use crate::traits::view_controller::PageType;
use crate::traits::view_model::*;
use crate::utils::event_loop::EventLoop as Events;
use crate::WidgetIds;

use conrod::backend::glium::glium::glutin::*;
use conrod::backend::glium::glium::Display;
use conrod::backend::glium::glium::*;
use conrod::backend::glium::Renderer;
use conrod::backend::winit::convert_event;
use conrod::glium::Surface;
use conrod::image::Map;
use conrod::*;
use glium::backend::glutin::DisplayCreationError;
use glium::texture::*;
use ::image::open;
use std::path::Path;
pub struct GameplayController {
    window_controller: WindowController,
    page_controller: Box<GameViewController>,
    page_model: Box<dyn GameViewModel>,
    ui: Ui,
    widget_ids: WidgetIds,
    events_loop: EventsLoop,
    width: u32,
    height: u32,
}

#[rustfmt::skip]
impl GameplayController {
    pub fn new(width: u32, height: u32, ui: Ui, widget_ids: WidgetIds) -> GameplayController {
        let window_controller = WindowController::new();
        let page_controller = GameBuilderController::new(&widget_ids);
        let page_model:  Box<dyn GameViewModel> = Box::new(GameBuilder::new(None));
        GameplayController {
            window_controller,
            page_controller,
            page_model,
            ui,
            widget_ids,
            events_loop: EventsLoop::new(),
            width,
            height,
        }
    }

    pub fn open_window(&self) -> Result<Display, DisplayCreationError> {
        let window_builder = WindowBuilder::new()
            .with_decorations(false)
            .with_resizable(false)
            .with_dimensions((self.width, self.height).into());
        let context_builder = ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        Display::new(window_builder, context_builder, &self.events_loop)
    }

    pub fn is_callback(&self, event: &Event) -> bool{
         if let glutin::Event::WindowEvent { event, .. } = event {
             match event {
                glutin::WindowEvent::CloseRequested | glutin::WindowEvent::KeyboardInput {
                    input: glutin::KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => return true,
                _ => ()
            }
        }
        false
    }

    pub fn render_loop(&mut self, display : Display, mut renderer: Renderer, image_map: Map<Texture2d>) {
        let mut events = Events::new();
        'render: loop {
            for event in events.next(&mut self.events_loop) {
                if let Some(event) = convert_event(event.clone(), &display) {
                    // println!("event ");
                    self.ui.handle_event(event);
                    events.needs_update();
                }
                if self.is_callback(&event) {
                   self.page_controller = match self.page_controller.get_type() {
                       PageType::Game => {
                           let game = self.page_model.get_model().downcast_ref::<Game>().unwrap();
                           let game = if game.is_finish() {
                               None
                           }
                           else {
                               Some(game.clone())
                           };
						   self.page_model = Box::new(GameBuilder::new(game));
						   GameBuilderController::new(&self.widget_ids)
					   },
                       _ => break 'render,
                   }
                }
            }
            if self.page_model.need_change_window() {
                self.page_controller = match self.page_controller.get_type() {
                    PageType::GameBuilder => {
                        let game_builder: &mut GameBuilder = self.page_model.get_model().downcast_mut::<GameBuilder>().unwrap();
						self.page_model = Box::new(game_builder.build());
						GameController::new(&self.widget_ids)
					},
                    _ => { 
                        let game = self.page_model.get_model().downcast_ref::<Game>().unwrap();
                        let game = if game.is_finish() {
                            None
                        }
                        else {
                            Some(game.clone())
                        };
                        self.page_model = Box::new(GameBuilder::new(game));
                        GameBuilderController::new(&self.widget_ids)
                    },
                }
            }
            let ui = &mut self.ui.set_widgets();
            self.window_controller.show(ui, &self.widget_ids);
            self.page_controller.show(&mut (*self.page_model), ui, &self.widget_ids);

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

    pub fn run(&mut self) {

        let rgba_image = open(&Path::new("assets/images/wood.jpg")).unwrap().to_rgba();
        let image_dimensions = rgba_image.dimensions();
        let background = RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), image_dimensions);
        let mut image_map = Map::<Texture2d>::new();

        let display = self.open_window().unwrap();
        let renderer = Renderer::new(&display).unwrap();

        self.window_controller.load_background(&mut image_map, &display, background);
        self.ui.fonts.insert_from_file(Path::new("assets/fonts/FiraSans-Regular.ttf")).unwrap();
        self.render_loop(display, renderer, image_map);
    }
}
