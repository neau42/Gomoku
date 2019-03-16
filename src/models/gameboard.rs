//! Gameboard

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Size of game board.
pub const SIZE: usize = 19;

#[derive(Debug,PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Stone {
    BLACK,
    WHITE,
	NOPE,
}

impl Stone {
    pub fn switch(&mut self) {
        *self = match *self {
            Stone::BLACK => Stone::WHITE,
            Stone::WHITE => Stone::BLACK,
            _ => return,
        }
    }
}

/// Stores game board information.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Gameboard {
	pub size: usize,
    pub cells: [[Stone; SIZE]; SIZE],
	pub possible_moves: [[bool; SIZE]; SIZE],
}

/// Creates a new game board.
impl Gameboard {
	pub fn new() -> Gameboard {
		Gameboard {
			size: SIZE,
			cells: [[Stone::NOPE; SIZE]; SIZE],
			possible_moves: [[false; SIZE]; SIZE],
		}
	}
}

impl Gameboard {
    pub fn make_move(&mut self, x: usize, y: usize, stone: Stone) -> bool {
		if self.cells[x][y] == Stone::NOPE {
			    self.update_possible_move(x, y);
				self.cells[x][y] = stone;
                return true;
        }
        false
    }

	pub fn eval(&self) -> isize {
		0
	}

    pub fn unmake_move(&mut self, x: usize, y: usize) {
        self.cells[x][y] = Stone::NOPE;
    }

	
	
	pub fn update_possible_move(&mut self, x: usize, y: usize) {
        let directions: [(isize, isize); 8] = [(0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1)];
        directions.iter().for_each(|(tmp_x, tmp_y)| {
			let tmp_x = *tmp_x + x as isize;
			let tmp_y = *tmp_y + y as isize;
			if tmp_x < 0 || tmp_x >= self.size as isize || tmp_y < 0 || tmp_y >= self.size as isize {
				return;
			}
			if self.cells[tmp_x as usize][tmp_y as usize] == Stone::NOPE {
				self.possible_moves[tmp_x as usize][tmp_y as usize] = true;
			}
		})
	}
	

	pub fn next_move(&self, last_move: Option<(usize, usize)>) -> Option<(usize, usize)> {
		let directions: [(isize, isize); 8] = [(0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1)];
		let range: Vec<usize> = (0..self.size as usize).collect();
		let starting_move: Option<(usize, usize)> = match last_move {
			Some(last_move) => {
				match last_move {
					_ if { last_move.0 == SIZE - 1 && last_move.1 == SIZE - 1} => None,
					_ if { last_move.0 == SIZE - 1} => Some((0, last_move.1 + 1)),
					_ => Some((last_move.0 + 1, last_move.1)),
				}
			},
			None => Some((0, 0)),
		};
		if (starting_move.is_none()) {
			return None;
		}
		let (starting_x, starting_y) = starting_move.unwrap();
		let mut selected_move: Option<(usize, usize)> = None;
		let is_neighbour = |x: usize, y: usize| -> bool {
			directions.iter().any(|(tmp_x, tmp_y)| {
				let tmp_x = *tmp_x + x as isize;
				let tmp_y = *tmp_y + y as isize;
				if tmp_x < 0 || tmp_x >= self.size as isize || tmp_y < 0 || tmp_y >= self.size as isize {// ou superieur a size
					return false;
				}
				let tmp_stone = self.cells[tmp_x as usize][tmp_y as usize];
				match tmp_stone {
					Stone::NOPE => false,
					_ => true,
				}
			})
		};
		range
			.iter()
			.filter(|y| **y >= starting_y)
			.any(|y| range
				.iter()
				.filter(|x| *y > starting_y || **x >= starting_x)
				.any(|x| {
					if self.cells[*x][*y] == Stone::NOPE && self.possible_moves[*x][*y] {
						selected_move = Some((*x, *y));
						return true;
					}
					false
				})
			);
		// dbg!(&selected_move);
		selected_move
	}
}

impl Gameboard {
}
