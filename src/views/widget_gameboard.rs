use crate::models::gameboard::*;
use conrod::{self, widget, color, Colorable, Sizeable, Borderable, Positionable, Widget};
use conrod::Scalar;
use conrod::Point;
use conrod::UiCell;
use conrod::widget::id::Id;
use conrod::color::Color;
// use conrod::widget::State;
use conrod::position::rect::Rect;

/// BOARD
#[derive(WidgetCommon_)]
pub struct Board<'a> {
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    board_state: &'a Gameboard,
    // old_x: usize
    // old_y: usize
}

impl<'a> Board<'a> {
    pub fn new(board_state: &'a Gameboard) -> Self {
        Board {
            common: widget::CommonBuilder::default(),
			board_state: board_state,
        }
    }

    pub fn boxe_is_empty(y: usize, x: usize) -> bool {
        // if board_state.
        true
    }
}

/// EVENT
#[derive(Debug, Copy, Clone, PartialEq)]
enum Interaction { Idle, Hover, Press }

#[derive(Clone, Debug)]
#[allow(missing_copy_implementations)]
pub struct InfoClick {
    pub is_click : u16,
    pub y: usize,
    pub x: usize,
}

impl InfoClick {
    /// `true` if the `Button` was clicked one or more times.
	///  pub fn was_clicked(self) -> bool { self.is_click > 0 }
    pub fn was_clicked(self) -> Option<(usize, usize)> {
        if (self.is_click == 0) {
            None
        }
        else {
            Some((self.y, self.x))
        }
    }
}

impl Iterator for InfoClick {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_click > 0 {
            self.is_click -= 1;
            Some(())
        } else {
            None
        }
    }
}

// fn get_mouse_pos(button_id: widget::Id, ui: &UiCell) -> (f64, f64) {
// 	 if let Some(mouse) = ui.widget_input(button_id).mouse() {
//         if mouse.buttons.left().is_down() {
//             let mouse_abs_xy = mouse.abs_xy();
//             // let clamped_x = inner_rect.x.clamp_value(mouse_abs_xy[0]);
//             // let clamped_y = inner_rect.y.clamp_value(mouse_abs_xy[1]);
//             // let (l, r, b, t) = inner_rect.l_r_b_t();
//             // new_x = map_range(clamped_x, l, r, min_x, max_x);
//             // new_y = map_range(clamped_y, b, t, min_y, max_y);
// 	return (mouse_abs_xy[0], mouse_abs_xy[1])
//         }
//     }
// 	(0.0, 0.0)

// }

fn interaction_and_times_triggered(button_id: widget::Id, ui: &UiCell) -> (Interaction, u16) {
    let input = ui.widget_input(button_id);
    let interaction = input.mouse().map_or(Interaction::Idle, |mouse| {
        let is_pressed =
            mouse.buttons.left().is_down()
            || ui.global_input().current.touch.values()
                 .any(|t| t.start.widget == Some(button_id));

        if is_pressed { Interaction::Press } else { Interaction::Hover }
    });
    let times_triggered = (input.clicks().left().count() + input.taps().count()) as u16;
    (interaction, times_triggered)
}

// fn interaction_and_times_triggered(button_id: widget::Id, ui: &UiCell) -> (Interaction, u16) {//, f64, f64) {
//     let input = ui.widget_input(button_id);
//     let interaction = input.mouse().map_or(Interaction::Idle, |mouse| {
//     // let (interaction, x, y) = input.mouse().map_or((Interaction::Idle, 0.0, 0.0), |mouse| {
//         let is_pressed =
//             mouse.buttons.left().is_down()
//             || ui.global_input().current.touch.values()
//                  .any(|t| t.start.widget == Some(button_id));
// 		// 		 		let mouse_abs_xy = mouse.abs_xy();
//         // if is_pressed { (Interaction::Press, mouse_abs_xy[0], mouse_abs_xy[1]) } else { (Interaction::Hover, mouse_abs_xy[0], mouse_abs_xy[1]) }
//         if is_pressed { (Interaction::Press) } else { (Interaction::Hover) }
//     });
//     let times_triggered = (input.clicks().left().count() + input.taps().count()) as u16;
//     // (interaction, times_triggered, x, y)
//     (interaction, times_triggered)
// }

///INDEXS
pub struct State {
    cell_index: CellIndex,
}

struct CellIndex {
    lines: conrod::widget::id::List,
	borders_background: conrod::widget::id::Id
}

impl CellIndex {
    pub fn new(mut generator: conrod::widget::id::Generator) -> Self {
        let mut cell_index = CellIndex {
            lines: conrod::widget::id::List::new(),
			borders_background: generator.next()
        };
        cell_index.lines.resize(19 * 23, &mut generator);//19*19: pierres + 19 * 3: lines + 9 hoshi + ...
        cell_index
    }
}

///IMPLEMENTATION
impl<'a> Widget for Board<'a> {
    type State = State;
    type Style = ();
    type Event = InfoClick;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            cell_index: CellIndex::new(id_gen)
        }
    }

    fn style(&self) -> Self::Style {
        ()
    }

    // Draw les pierres deja poser
    // fn draw_boxes(mut self, &widget::UpdateArgs<Self> ) {
    //     let widget::UpdateArgs { id, state, rect, ui, .. } = args;
    // }

    fn update(mut self, args: widget::UpdateArgs<Self>) -> Self::Event {
		let widget::UpdateArgs { id, state, rect, ui, .. } = args;
		// let (x, y) = get_mouse_pos(id, ui);
        let (interaction, times_triggered) = interaction_and_times_triggered(id, ui);
		if (interaction != Interaction::Idle) {
			println!("interaction: {:?}", interaction);
		}
        // let (interaction, times_triggered, x, y) = interaction_and_times_triggered(id, ui);

		// println!("-----------> {}, {} ",x, y);
        draw_lines(self.board_state.size, id, &state, rect, ui);
		draw_hoshi(self.board_state.size, id, &state, rect, ui);
		draw_stones(self.board_state, id, &state, rect, ui);
        // self.draw_boxes(&args);
        // if (true/*n'est pas deja afficher(deja la previous dans la case) +  est une empty box, */) {
        //     self.draw_preview_boxe();
        // }

		InfoClick {
            is_click: times_triggered,
            x: 42 as usize,
            y: 42 as usize,
        }
    }
}

/// draw gomoku gameboard lines
fn draw_lines(size: usize, id: Id, state: &State, rect: Rect, ui: &mut UiCell) {
	let x2 = rect.x.start + rect.w();
	
	//draw white lines border horizontal
	for i in 0..size {
		let y = rect.y.start + i as f64 / (size - 1) as f64 * rect.h();
		conrod::widget::primitive::line::Line::new([rect.x.start, y - 1.0], [x2, y - 1.0])
			.x_y_relative_to(id, 0.0, 0.0)
			.color(color::WHITE)
			.thickness(0.3)
			.set(state.cell_index.lines[i+(size*2)], ui);
	}
	let x2 = rect.x.end;
	let y2 = rect.y.end;
	
	//draw black lines horizontal and vertical
	for i in 0..size {
		let x = rect.x.start + i as f64 / (size - 1) as f64 * rect.w();
		let y = rect.y.start + i as f64 / (size - 1) as f64 * rect.h();

		conrod::widget::primitive::line::Line::new([rect.x.start , y], [x2, y])
			.x_y_relative_to(id, 0.0, 0.0)
			.color(color::BLACK)
			.thickness(1.0)
			.set(state.cell_index.lines[i], ui);

		conrod::widget::primitive::line::Line::new([x, rect.y.start], [x, y2])
			.x_y_relative_to(id, 0.0, 0.0)
			.color(color::BLACK)
			.thickness(1.0)
			.set(state.cell_index.lines[i+size], ui);
		}
	}

/// draw hoshi on gameboard
fn draw_hoshi(size: usize, id: Id, state: &State, rect: Rect, ui: &mut UiCell) {
	let mut cmpt = 0;
	let stone_size = (rect.x.end - rect.x.start) / (size - 1) as f64;
	for i in [3, (size - 1) / 2, size - 4].iter() {
		for j in [3, (size - 1) / 2, size - 4].iter() {
			conrod::widget::primitive::shape::rectangle::Rectangle::fill([15.0, 15.0])
				.x_y_relative_to(
					id,
					rect.x.start + (*i as f64 * stone_size),
					rect.y.start + (*j as f64 * stone_size)
					)
				.color(color::BLACK)
				.set(state.cell_index.lines[cmpt+(size * 3)], ui);
				cmpt +=1;
		}
	}
}

/// draw all stones presents on board
fn draw_stones(board_state: & Gameboard, id: Id, state: &State, rect: Rect, ui: &mut UiCell) {

	for i in 0..board_state.size * board_state.size {
		match board_state.cells[i/board_state.size][i%board_state.size] {
			Stone::WHITE =>	draw_one_stone(
				[i / board_state.size, i % board_state.size],
				color::WHITE,
				board_state.size,
				id, rect, ui, state.cell_index.lines[19*4 + i]),
			Stone::BLACK => draw_one_stone(
				[i / board_state.size, i % board_state.size],
				color::BLACK,
				board_state.size,
				id, rect, ui, state.cell_index.lines[19*4 + i]),
			_ => (),
		}
	}
}

/// draw one stone on board in [ind[0]][ind[1]] 
fn draw_one_stone(ind: [usize; 2],color: Color, size: usize, id: Id, rect: Rect, ui: &mut UiCell, line_id: Id) {
	let stone_size = (rect.x.end - rect.x.start) / (size - 1) as f64;

	let pos = [
			ind[0] as f64 * stone_size - rect.x.end,
			rect.y.end - ind[1] as f64 * stone_size
			];

		conrod::widget::primitive::shape::circle::Circle::fill(stone_size/2.0)
			.x_y_relative_to(
					id,
					pos[0],
					pos[1])
				.color(color)
				.set(line_id, ui);
}
