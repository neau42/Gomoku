use crate::models::gameboard::*;
use conrod::color::Color;
use conrod::position::rect::Rect;
use conrod::widget::id::Id;
use conrod::UiCell;
use conrod::{self, color, widget, Colorable, Positionable, Widget};

/// BOARD
#[derive(WidgetCommon_)]
pub struct Board<'a> {
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    board_state: &'a Gameboard,
    is_human: bool,
    color: Color,
	show_all_values: bool,
	all_values: &'a Vec<(usize, usize, isize)>,
}

impl<'a> Board<'a> {
    pub fn new(board_state: &'a Gameboard, stone: u8, all_values: &'a Vec<(usize, usize, isize)>, show_all_values: bool, is_human: bool) -> Self {
        let color = match stone {
            BLACK => color::BLACK,
            _ => color::WHITE,
        };
        Board {
            common: widget::CommonBuilder::default(),
            board_state,
            is_human,
            color,
			show_all_values,
			all_values,
        }
    }
}

/// EVENT
#[derive(Debug, Copy, Clone, PartialEq)]
enum Interaction {
    Idle,
    Hover,
    Press,
}

#[derive(Clone, Debug)]
#[allow(missing_copy_implementations)]
pub struct InfoClick {
    pub is_click: u16,
    pub x: usize,
    pub y: usize,
}

impl InfoClick {
    pub fn was_clicked(self) -> Option<(usize, usize)> {
        if self.is_click == 0 {
            None
        } else {
            Some((self.x, self.y))
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

/// get board idx from mouse position
pub fn get_cell(x: f64, y: f64, rect: Rect) -> Option<(usize, usize)> {
    let size_px = rect.w();
    let map_size = SIZE;
    let semi_cell_size = size_px / map_size as f64 / 2.0;

    if x >= rect.x.start - semi_cell_size
        && x < rect.x.end + semi_cell_size
        && y >= rect.y.start - semi_cell_size
        && y < rect.y.end + semi_cell_size
    {
        let stone_x =
            ((x - rect.x.start + semi_cell_size) / size_px * (map_size - 1) as f64) as usize;
        let stone_y =
            ((rect.y.end - y + semi_cell_size) / size_px * (map_size - 1) as f64) as usize;
        Some((stone_x, stone_y))
    } else {
        None
    }
}

fn get_mouse_event(
    rect: Rect,
    button_id: conrod::widget::Id,
    ui: &UiCell,
) -> (Interaction, u16, usize, usize) {
    let input = ui.widget_input(button_id);

    let (interaction, x_mouse, y_mouse) =
        input
            .mouse()
            .map_or((Interaction::Idle, 0.0, 0.0), |mouse| {
                let is_pressed = mouse.buttons.left().is_down();
                let mouse_abs_xy = mouse.abs_xy();
                if is_pressed {
                    (Interaction::Press, mouse_abs_xy[0], mouse_abs_xy[1])
                } else {
                    (Interaction::Hover, mouse_abs_xy[0], mouse_abs_xy[1])
                }
            });

    let is_click = (input.clicks().left().count() + input.taps().count()) as u16;
    match interaction {
        Interaction::Idle => (interaction, 0, 0, 0),
        _ => match get_cell(x_mouse, y_mouse, rect) {
            Some((x, y)) => (interaction, is_click, x, y),
            None => (Interaction::Idle, 0, 0, 0),
        },
    }
}

///INDEXS
pub struct State {
    cell_index: CellIndex,
}

struct CellIndex {
    lines: conrod::widget::id::List,
    stones: conrod::widget::id::List,
    hoshis: conrod::widget::id::List,
	values: conrod::widget::id::List,
    last_move: conrod::widget::id::Id,

}

impl CellIndex {
    pub fn new(mut generator: conrod::widget::id::Generator, size: usize) -> Self {
        let mut cell_index = CellIndex {
            lines: conrod::widget::id::List::new(),
            stones: conrod::widget::id::List::new(),
            hoshis: conrod::widget::id::List::new(),
            values: conrod::widget::id::List::new(),
            last_move: generator.next(),
        };
        cell_index.lines.resize(size * 3, &mut generator);
        cell_index.stones.resize(size * size, &mut generator);
        cell_index.hoshis.resize(9, &mut generator);
        cell_index.values.resize(size * size, &mut generator);

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
            cell_index: CellIndex::new(id_gen, SIZE),
        }
    }

    fn style(&self) -> Self::Style {
    }

    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs {
            id,
            state,
            rect,
            ui,
            ..
        } = args;
        let size = SIZE;

        draw_lines(size, id, &state, rect, ui);
        draw_hoshi(size, id, &state, rect, ui);
        draw_stones(self.board_state, id, &state, rect, ui, &self.all_values, self.show_all_values);
        let (interaction, is_click, x, y) = get_mouse_event(rect, id, ui);
        if !self.is_human {
            return InfoClick { is_click: 0, x, y };
        }
        let shift = y * 2;
        if (interaction == Interaction::Hover || interaction == Interaction::Press) && (self.board_state.cells[x] >> shift) & 0b11 == 0 {
            let color = match interaction {
                Interaction::Hover => self.color.with_alpha(0.5),
                _ => self.color,
            };
            draw_one_stone(
                [x, y],
                color,
                id,
                rect,
                ui,
                state.cell_index.stones[x + y * size],
            );
        }
        InfoClick { is_click, x, y }
    }
}

/// draw gomoku gameboard lines
fn draw_lines(size: usize, id: Id, state: &State, rect: Rect, ui: &mut UiCell) {
    let x2 = rect.x.start + rect.w();

    //draw white lines_border horizontal
    for i in 0..size {
        let y = rect.y.start + i as f64 / (size - 1) as f64 * rect.h();
        conrod::widget::primitive::line::Line::new([rect.x.start, y - 1.0], [x2, y - 1.0])
            .x_y_relative_to(id, 0.0, 0.0)
            .color(color::WHITE)
            .thickness(0.3)
            .graphics_for(id)
            .set(state.cell_index.lines[i + (size * 2)], ui);
    }
    let x2 = rect.x.end;
    let y2 = rect.y.end;

    //draw black lines horizontal and vertical
    for i in 0..size {
        let x = rect.x.start + i as f64 / (size - 1) as f64 * rect.w();
        let y = rect.y.start + i as f64 / (size - 1) as f64 * rect.h();

        conrod::widget::primitive::line::Line::new([rect.x.start, y], [x2, y])
            .x_y_relative_to(id, 0.0, 0.0)
            .color(color::BLACK)
            .thickness(1.0)
            .graphics_for(id)
            .set(state.cell_index.lines[i], ui);

        conrod::widget::primitive::line::Line::new([x, rect.y.start], [x, y2])
            .x_y_relative_to(id, 0.0, 0.0)
            .color(color::BLACK)
            .thickness(1.0)
            .graphics_for(id)
            .set(state.cell_index.lines[i + size], ui);
    }
}

/// draw hoshi on gameboard
fn draw_hoshi(size: usize, id: Id, state: &State, rect: Rect, ui: &mut UiCell) {
    let mut cmpt = 0;
    let half_w = rect.w() / 2.0;
    let stone_size = rect.w() / (size - 1) as f64;
    for i in [3, (size - 1) / 2, size - 4].iter() {
        for j in [3, (size - 1) / 2, size - 4].iter() {
            conrod::widget::primitive::shape::rectangle::Rectangle::fill([15.0, 15.0])
                .x_y_relative_to(
                    id,
                    (*i as f64 * stone_size) - half_w,
                    (*j as f64 * stone_size) - half_w,
                )
                .color(color::BLACK)
                .graphics_for(id)
                .set(state.cell_index.hoshis[cmpt], ui);
            cmpt += 1;
        }
    }
}

/// draw all stones presents on board
fn draw_stones(board_state: &Gameboard, id: Id, state: &State, rect: Rect, ui: &mut UiCell, all_values: &Vec<(usize, usize, isize)>, show_all_value: bool) {
    for i in 0..SIZE * SIZE {
		match get_stone!(board_state.cells[i % SIZE], i / SIZE) {
			WHITE => draw_one_stone(
				[i % SIZE, i / SIZE],
				color::WHITE,
				id,
				rect,
				ui,
				state.cell_index.stones[i],
			),
			BLACK => draw_one_stone(
				[i % SIZE, i / SIZE],
				color::BLACK,
				id,
				rect,
				ui,
				state.cell_index.stones[i],
			),
			_ => (),
		}
	}

	if show_all_value {
		let mut i = 0;
		for elem in all_values {
			display_value( [elem.0, elem.1],
					color::LIGHT_BROWN,
					id,
					rect,
					ui,
					state.cell_index.values[i], elem.2
			);
		i+=1;
		}
	}
    let half_w = rect.w() / 2.0;
    let stone_size = rect.w() / (SIZE - 1) as f64;
    if let Some((x, y)) = board_state.last_move {
        conrod::widget::primitive::shape::rectangle::Rectangle::fill([10.0, 10.0])
        .x_y_relative_to(
            id,
            (x as f64 * stone_size) - half_w,
            ((SIZE - 1 - y) as f64 * stone_size) - half_w,
            )
            .color(color::LIGHT_BROWN)
            .graphics_for(id)
            .set(state.cell_index.last_move, ui);
    }
}

/// display values eval on board
fn display_value(ind: [usize; 2],
    color: Color,
    id: Id,
    rect: Rect,
    ui: &mut UiCell,
    cell_id: Id, value: isize) {

	let stone_size = (rect.x.end - rect.x.start) / (SIZE - 1) as f64;
    let pos = [
        ind[0] as f64 * stone_size - rect.w() / 2.0,
        rect.w() / 2.0 - ind[1] as f64 * stone_size,
    ];

	if ind[0] % 2 == 0 {
		conrod::widget::primitive::text::Text::new(value.to_string().as_str())
			.x_y_relative_to(id, pos[0], pos[1]- 6.0)
			.color(color)
			.graphics_for(id)
			.font_size(14)
			.set(cell_id, ui);
	} else {
		conrod::widget::primitive::text::Text::new(value.to_string().as_str())
			.x_y_relative_to(id, pos[0], pos[1] + 4.0)
			.color(color)
			.graphics_for(id)
			.font_size(14)
			.set(cell_id, ui);

	}

}

/// draw one stone on board in [ind[0]][ind[1]]
fn draw_one_stone(
    ind: [usize; 2],
    color: Color,
    id: Id,
    rect: Rect,
    ui: &mut UiCell,
    cell_id: Id,
) {
    let stone_size = (rect.x.end - rect.x.start) / (SIZE - 1) as f64;
    let pos = [
        ind[0] as f64 * stone_size - rect.w() / 2.0,
        rect.w() / 2.0 - ind[1] as f64 * stone_size,
    ];
    conrod::widget::primitive::shape::circle::Circle::fill(stone_size / 2.0)
        .x_y_relative_to(id, pos[0], pos[1])
        .color(color)
        .graphics_for(id)
        .set(cell_id, ui);
}
