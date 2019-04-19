use crate::models::game::GameResult;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use crate::eval::*;
use crate::models::ia::IA;
use std::collections::HashMap;

/// Size of game board.
#[cfg(feature = "size13")]
pub const SIZE: usize = 13;

#[cfg(feature = "size15")]
pub const SIZE: usize = 15;

#[cfg(feature = "size17")]
pub const SIZE: usize = 17;

#[cfg(not(any(feature = "size13", feature = "size15", feature = "size17")))]
pub const SIZE: usize = 19;


pub const NOPE: u8 = 0b00;
pub const BLACK: u8 = 0b01;
pub const WHITE: u8 = 0b10;

pub const WHITE_CAPTURE: u8 = WHITE | BLACK << 2 | BLACK << 4 | WHITE << 6;
pub const BLACK_CAPTURE: u8 = BLACK | WHITE << 2 | WHITE << 4 | BLACK << 6;

pub const BLACK_TREES: [u16; 4] = [
	NOPE as u16 | (BLACK as u16) << 2 | (BLACK as u16) << 4 | (BLACK as u16) << 6 | (NOPE as u16) << 8 | (NOPE as u16) << 10,
	NOPE as u16 | (BLACK as u16) << 2 | (BLACK as u16) << 4 | (NOPE as u16) << 6 | (BLACK as u16) << 8 | (NOPE as u16) << 10,
	NOPE as u16 | (BLACK as u16) << 2 | (NOPE as u16) << 4 | (BLACK as u16) << 6 | (BLACK as u16) << 8 | (NOPE as u16) << 10,
	NOPE as u16 | (NOPE as u16) << 2 | (BLACK as u16) << 4 | (BLACK as u16) << 6 | (BLACK as u16) << 8 | (NOPE as u16) << 10,
	];
pub const WHITE_TREES: [u16; 4] = [
	NOPE as u16 | (WHITE as u16) << 2 | (WHITE as u16) << 4 | (WHITE as u16) << 6 | (NOPE as u16) << 8 | (NOPE as u16) << 10,
	NOPE as u16 | (WHITE as u16) << 2 | (WHITE as u16) << 4 | (NOPE as u16) << 6 | (WHITE as u16) << 8 | (NOPE as u16) << 10,
	NOPE as u16 | (WHITE as u16) << 2 | (NOPE as u16) << 4 | (WHITE as u16) << 6 | (WHITE as u16) << 8 | (NOPE as u16) << 10,
	NOPE as u16 | (NOPE as u16) << 2 | (WHITE as u16) << 4 | (WHITE as u16) << 6 | (WHITE as u16) << 8 | (NOPE as u16) << 10,
	];


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Priority {
    BlackWin,
    BlackWin1,
    BlackWin2,
    BlackPossibleWin1,
	BlackPossibleWin2,
	BlackPossibleWin2Capturable,
	WhiteWin,
    WhiteWin1,
    WhiteWin2,
    WhitePossibleWin1,
    WhitePossibleWin2,
	WhitePossibleWin2Capturable,
	Other,
}


impl Priority {
	pub fn get_index_of(&self) -> usize {
		match self {
			Priority::BlackWin => 0,
			Priority::BlackWin1 => 1,
			Priority::BlackWin2 => 2,
			Priority::BlackPossibleWin1 => 3,
			Priority::BlackPossibleWin2 => 4,
			Priority::BlackPossibleWin2Capturable => 5,
			Priority::WhiteWin => 6,
			Priority::WhiteWin1 => 7,
			Priority::WhiteWin2 => 8,
			Priority::WhitePossibleWin1 => 9,
			Priority::WhitePossibleWin2 => 10,
			Priority::WhitePossibleWin2Capturable => 11,
			Priority::Other => 12,
		}
	}
}

#[derive(Debug, Eq, Clone)]
pub struct Gameboard {
    pub cells: [u64; SIZE],
	pub possible_moves: [u32; SIZE],
    pub selected_move: Option<(usize, usize)>,
    pub last_move: Option<(usize, usize)>,
	pub black_captures: u8,
	pub white_captures: u8,
	pub value: isize,
	pub priority: Priority,
	pub result: Option<GameResult>,
    pub waiting_winning_move: Option<(usize, usize)>,
}

impl Gameboard {
	pub fn new() -> Gameboard {
		Gameboard {
			cells: [0; SIZE],
			possible_moves: [0; SIZE],
            selected_move: None,
            last_move: None,
			black_captures: 0,
			white_captures: 0,
			value: 0,
			priority: Priority::Other,
			result: None,
			waiting_winning_move: None,
		}
	}

	pub fn is_finish(&self) -> bool {
		self.result.is_some() && self.waiting_winning_move.is_none()
    }
}
impl Gameboard {
	pub fn count_tree(&self, tree_lines: [u32; 4], stone: u8) -> u8 {
		let tree_forms: [u16; 4] = get_tree_forms!(stone);
		tree_lines.iter().fold(0, |nbr_tree, line| {
			let is_tree: bool = (0..6).any(|range| {
				let line_to_check: u32 = line >> (range * 2);
				tree_forms.contains(&(concat_stones!(line_to_check, 6) as u16))
			});
			if is_tree {
				nbr_tree + 1
			}
			else {
				nbr_tree
			}
		})
	}

	pub fn count_capture(&mut self, capture_lines: [(u8, (isize, isize)); 8], x: usize, y: usize, stone: u8) -> u8 {
		let capture_form: u8 = get_capture_form!(stone);
		capture_lines.iter().fold(0, |nbr_capture, (line, coef)| {
			if *line == capture_form {
				self.cells[(x as isize + coef.0) as usize] &= clear_stone!((y as isize + coef.1) as usize);
				self.cells[(x as isize + 2 * coef.0) as usize] &= clear_stone!((y as isize + 2 * coef.1) as usize);
				self.possible_moves[(x as isize + coef.0) as usize] |= set_move!((y as isize + coef.1) as usize);
				self.possible_moves[(x as isize + 2 * coef.0) as usize] |= set_move!((y as isize + 2 * coef.1) as usize);
				return nbr_capture + 1;
			}
			nbr_capture
		})
	}

	pub fn try_make_move(&mut self, x: isize, y: isize, stone: u8) -> bool {
		let x_min = (x - 5).max(0) as usize;
		let x_max = (x + 5).min(SIZE as isize - 1) as usize;
		let y_min = (y - 5).max(0) as usize;
		let y_max = (y + 5).min(SIZE as isize - 1) as usize;
		let diago_up_left = (y as usize - y_min).min(x as usize - x_min);
		let diago_up_right = (y as usize - y_min).min(x_max - x as usize);
		let diago_down_right = (y_max - y as usize).min(x_max - x as usize);
		let diago_down_left = (y_max - y as usize).min(x as usize - x_min);

		let capture_lines: [(u8, (isize, isize)); 8] = capture_lines!(self.cells, x as usize, x_min, x_max, y as usize, y_min, y_max, diago_up_left, diago_down_right, diago_down_left, diago_up_right);
		let nbr_capture = self.count_capture(capture_lines, x as usize, y as usize, stone);
		if nbr_capture == 0 {
			let tree_lines: [u32; 4] = tree_lines!(self.cells, x as usize, x_min, x_max, y as usize, y_min, y_max, diago_up_left, diago_down_right, diago_down_left, diago_up_right);
			let nbr_tree = self.count_tree(tree_lines, stone);
			return nbr_tree < 2;
		}
		match stone {
			BLACK => self.black_captures += nbr_capture * 2,
			_ => self.white_captures += nbr_capture * 2,
		}
		true
	}

	pub fn make_move(&mut self, x: usize, y: usize, stone: u8) -> bool {
		let tmp_state = self.clone();
		if !self.is_finish() && get_stone!(self.cells[x], y) == NOPE {
			self.cells[x] |= set_stone!(y, stone);
			self.update_possible_move(x as isize, y as isize);
			if self.try_make_move(x as isize, y as isize, stone) && self.update_result(x, y, stone) {
				self.last_move = Some((x, y));
				self.selected_move = None;
				return true;
			}
			*self = tmp_state;
        }
        false
    }
	
	pub fn update_possible_move(&mut self, x: isize, y: isize) {
		let min_x = (x - 1).max(0) as usize;
		let min_y = (y - 1).max(0) as usize;
		let max_x = (x + 1).min(SIZE as isize - 1) as usize;
		let max_y = (y + 1).min(SIZE as isize - 1) as usize;

		let x = x as usize;
		let y = y as usize;
		let moves = [(min_x, y), (min_x, min_y), (min_x, max_y), (max_x, y), (max_x, min_y), (max_x, max_y), (x, min_y), (x, max_y)];
		moves
			.iter()
			.for_each(|new_move| {
				if get_stone!(self.cells[new_move.0], new_move.1) == NOPE {
					self.possible_moves[new_move.0 as usize] |= set_move!(new_move.1)
				}
			})
	}
	
	pub fn expand(&self) -> Vec<(usize, usize)> {
		(0..SIZE)
			.flat_map(|y| {
				(0..SIZE)
				.filter(move |&x| self.possible_moves[x] >> y & 0b1 == 1 && get_stone!(self.cells[x], y) == NOPE)
				.map(move |x| (x, y))
			})
		.collect()
	}

	pub fn update_result(&mut self, x: usize, y: usize, stone: u8) -> bool {
		if self.black_captures >= 10 {
			self.waiting_winning_move = None;
			self.result = Some(GameResult::BlackWin);
		}
		else if self.white_captures >= 10 {
			self.waiting_winning_move = None;
			self.result = Some(GameResult::WhiteWin);
		}
		else {
			if let Some(winning_move) = self.waiting_winning_move {
				if winning_move != (x, y) {
					let tmp_result = self.result.clone();
					self.result = None;
					self.update_result(winning_move.0, winning_move.1, opposite_stone!(stone));
					if self.result == tmp_result {
						return false;
					}
					self.waiting_winning_move = None;
				}
			}
			let x_min = (x as isize - 5).max(0) as usize;
			let x_max = (x + 5).min(SIZE - 1);
			let y_min = (y as isize  - 5).max(0) as usize;
			let y_max = (y + 5).min(SIZE - 1);

			let diago_up_left = (y as usize - y_min).min(x as usize - x_min);
			let diago_up_right = (y as usize - y_min).min(x_max - x as usize);
			let diago_down_right = (y_max - y as usize).min(x_max - x as usize);
			let diago_down_left = (y_max - y as usize).min(x as usize - x_min);
			let lines: [u32; 4] = tree_lines!(self.cells, x as usize, x_min, x_max, y as usize, y_min, y_max, diago_up_left, diago_down_right, diago_down_left, diago_up_right);
			let (win_align, win_result) =  if stone == WHITE {
				(WHITE_5_ALIGN, GameResult::WhiteWin)
			}
			else {
				(BLACK_5_ALIGN, GameResult::BlackWin)
			};
			lines.iter().any(|line| {
				(0..8).any(|range| {
					let tmp_line: u16 = concat_stones!((line >> (range * 2)) as u32, 5) as u16;
					if win_align == tmp_line {
						check_winning!(self, x, y, win_result, stone)
					}
					else {
						false
					}
				})
			});
		}
		true
	}
}

impl PartialOrd for Gameboard {
    fn partial_cmp(&self, other: &Gameboard) -> Option<Ordering> {
        other.value.partial_cmp(&self.value)
    }
}

impl PartialEq for Gameboard {
    fn eq(&self, other: &Gameboard) -> bool {
        self.cells == other.cells && self.black_captures == other.black_captures && self.white_captures == other.white_captures
    }
}

impl Hash for Gameboard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cells.hash(state);
        self.black_captures.hash(state);
        self.white_captures.hash(state);
    }
}
