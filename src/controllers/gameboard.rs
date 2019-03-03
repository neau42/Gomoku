//! Gameboard controller.

use conrod::UiCell;
use std::collections::HashMap;
use conrod::widget::id::Id;

use crate::WidgetIds;
use crate::traits::view_controller::*;
use crate::traits::view_model::GameViewModel;
use crate::models::game_info::*;
use glutin::WindowEvent::CursorMoved;

use conrod::backend::glium::glium::glutin::*;
use conrod::backend::glium::glium::*;

use crate::views::gameboard::GameboardView;
use crate::models::gameboard::Gameboard;
use crate::models::gameboard::Stone;


enum GameboardEvent {
	// return_button(fn()),
	grid(fn()),

	
}

pub struct GameboardController {
	pub view: GameboardView,
    cursor_pos: [f64; 2],
    release_position: [f64; 2],
	events: HashMap<Id, GameboardEvent>,

	test_switch_player: bool,
}

impl GameboardController {
  pub fn put_stone(&mut self, model: &mut Box<dyn GameViewModel>, ui: &mut UiCell, widget_ids: &WidgetIds) -> bool {
	  let model: &mut Gameboard = match model.get_model().downcast_mut::<Gameboard>() {
            Some(model) => model,
            None => panic!("&GameViewModel isn't a Gameboard!"),
        };

		model.selected_stone = self.get_cell(self.cursor_pos[0], self.cursor_pos[1], model);
		match model.selected_stone {
			Some(x_y) => {
				if model.cells[x_y[0]][x_y[1]] == Stone::NOPE {
					if self.test_switch_player == true { //TEMP
						model.cells[x_y[0]][x_y[1]] = Stone::BLACK; }
					else {
						model.cells[x_y[0]][x_y[1]] = Stone::WHITE; }
					self.test_switch_player = !self.test_switch_player;
					println!("stone put on X: {}, Y: {}", x_y[0], x_y[1]);
					return true;
				}
			},
			None => (),
		}
		false
	}

	pub fn get_cell(&mut self, x: f64, y: f64, model: &mut Gameboard) -> Option<[usize; 2]> {
		let size_px = self.view.size;
		let map_position = self.view.position;
		let map_size = model.size;
		let semi_cell_size = size_px / map_size as f64 / 2.0;

		// Check that coordinates are inside board boundaries.
		if x >= map_position[0] - semi_cell_size
			&& x < size_px + map_position[0] + semi_cell_size
			&& y >= map_position[1] - semi_cell_size
			&& y < size_px + map_position[1] + semi_cell_size {
			let stone_x = ((x - map_position[0] + semi_cell_size) / size_px * (map_size - 1) as f64) as usize;
			let stone_y = ((y - map_position[1] + semi_cell_size) / size_px * (map_size - 1) as f64) as usize;
			println!("stone x: {} y: {}", stone_x, stone_y);
			return Some([stone_x, stone_y])
		}
		None
	}

	pub fn set_events(&mut self, widget_ids: &WidgetIds) {
		// println!("GameboardController: set_events");
		 self.events.insert(widget_ids.grid, GameboardEvent::grid(|| {
            // model.change_window();
        }));
	// self.events.insert(widget_ids.dropdown_button_game_mode, GameboardEvent::return_button(|| {
    //         println!("return_button!");
    //     }));

	}
}


impl GameViewController for GameboardController {
    fn new(widget_ids: &WidgetIds) -> Box<GameboardController> {
        let view = GameboardView::new();
		println!("new GameboardController!");
        let controller = GameboardController {
			view,
			cursor_pos: [0.0; 2],
			release_position: [0.0; 2],
			events: HashMap::new(),

			test_switch_player: false,
			};
        // controller.set_events(widget_ids);
        Box::new(controller)
    }

	fn check_event(&mut self, event: &Event, model: &mut Box<dyn GameViewModel>, ui: &mut UiCell, widget_ids: &WidgetIds) {
		println!("check_event gameboard!");
		 match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                glutin::WindowEvent::CursorMoved {position, ..} => {
					println!("x = {}, y = {}", position.x, position.y);
					self.cursor_pos[0] = position.x;
					self.cursor_pos[1] = position.y;
				}
				glutin::WindowEvent::MouseInput {button: MouseButton::Left, state: ElementState::Released, ..} => {
					println!("release click");
					self.put_stone(model, ui, widget_ids);
					self.release_position = self.cursor_pos;
				}
				_ => (),
				}
			}
			_ => (),
		}
	}

    fn show(&self, model:  &mut Box<dyn GameViewModel>, ui: &mut UiCell, widget_ids: &WidgetIds) {
		let model: &mut Gameboard = match model.get_model().downcast_mut::<Gameboard>() {
            Some(model) => model,
            None => panic!("&GameViewModel isn't a Gameboard!"),
        };
		// self.view.display_canvas(ui, widget_ids);
        self.view.display_grid(model, ui, widget_ids);
        // self.view.draw_map(model, ui, widget_ids);
	}

    fn get_type(&self) -> PageType {
        PageType::Gameboard
    }
}
