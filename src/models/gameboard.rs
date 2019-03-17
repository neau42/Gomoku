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
	pub fn eval(&self) -> isize {
		0
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
	pub fn make_move(&mut self, x: usize, y: usize, stone: Stone) -> bool {
		if self.cells[x][y] == Stone::NOPE && self.check_double_tree(x, y, stone) {
				let nbr_capture = apply_capture();
				self.cells[x][y] = stone;
			    self.update_possible_move(x, y);
                return true;
        }
        false
    }

    pub fn unmake_move(&mut self, x: usize, y: usize) {
        self.cells[x][y] = Stone::NOPE;
    }

}

impl Gameboard {
    // True if capture is possible
    pub fn check_capture(&self, x: usize, y: usize, actual_stone: Stone) -> bool {
        let directions: [(isize, isize); 8] = [(0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1)];

        directions.iter().any(|(tmp_x, tmp_y)| {
            (1..=3 as isize).all(|i| {
                let tmp_x = *tmp_x  * i + x as isize;
                let tmp_y = *tmp_y * i + y as isize;
                if tmp_x < 0 || tmp_x >= self.size as isize || tmp_y < 0 || tmp_y >= self.size as isize {
                    return false;
                }
                let tmp_stone = self.cells[tmp_x as usize][tmp_y as usize];
                match i {
                    1 | 2 => tmp_stone != actual_stone && tmp_stone != Stone::NOPE,
                    _ => tmp_stone == actual_stone,
                }
            })
        })
	}

	pub fn check_double_tree(&self, x: usize, y: usize, actual_stone: Stone) -> bool {
        let directions: [(isize, isize); 4] = [(0,1), (1,0), (1,1), (1,-1)];

        let closure = |tmp_x: isize, tmp_y: isize| -> Vec<Stone> {
            (1..=5 as isize).filter_map(|i| {
                let tmp_x = tmp_x  * i + x as isize;
                let tmp_y = tmp_y * i + y as isize;
                if tmp_x < 0 || tmp_x >= self.size as isize || tmp_y < 0 || tmp_y >= self.size as isize {
                    return None;
                }
                Some(self.cells[tmp_x as usize][tmp_y as usize])
            }).collect()
        };
        
        let nbr_tree = directions.iter().fold(0, |nbr_tree, (tmp_x, tmp_y)| {
            let right_side = closure(*tmp_x, *tmp_y);
            let mut left_side = closure(tmp_x * -1, tmp_y * -1);
            left_side.reverse();
            let line = [&left_side[..], &vec![actual_stone][..], &right_side[..]].concat();
            let len = line.len();
            if len < 6 {
                return nbr_tree;
            }
            let is_tree: bool = (0..=(len - 6)).any(|i| {
                line[i] == Stone::NOPE
                && line[i + 5] == Stone::NOPE
                && line[(i + 1)..(i + 5)].iter()
                .fold(0, |sum, stone| {
                    match *stone {
                        otherstone if otherstone == actual_stone => sum + 1,
                        Stone::NOPE => sum + 2,
                        _ => sum + 3,
                    }
                }) == 5
            });
            if is_tree {
                nbr_tree + 1
            }
            else {
                nbr_tree
            }
        });
        println!("nbr_tree = {}", nbr_tree);
        nbr_tree >= 2
	}
}
