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
	pub value: isize,
	pub possible_moves: [[bool; SIZE]; SIZE],
}

/// Creates a new game board.
impl Gameboard {
	pub fn new() -> Gameboard {
		Gameboard {
			size: SIZE,
			cells: [[Stone::NOPE; SIZE]; SIZE],
			value: 0,
			possible_moves: [[false; SIZE]; SIZE],
		}
	}
}

pub fn eval_value(cells: &[[Stone; SIZE]; SIZE], x_orig: isize, y_orig: isize, stone: &Stone) -> isize {
	let mut value = 0;
	let x_min = (x_orig - 5).max(0);
	let x_max = (x_orig + 5).min(SIZE as isize - 1);
	let y_min = (y_orig - 5).max(0);
	let y_max = (y_orig + 5).min(SIZE as isize - 1);

	let horizontal: Vec<Stone> = (x_min..=x_max).map(|x| cells[x as usize][y_orig as usize]).collect();
	let vertical: Vec<Stone> = (y_min..=y_max).map(|y| cells[x_orig as usize][y as usize]).collect();

	let len_origin_min = (y_orig - y_min).min(x_orig - x_min);
	let len_origin_max = (y_max - y_orig).min(x_max - x_orig);

	let diag1: Vec<Stone> = ((x_orig-len_origin_min)..=(x_orig + len_origin_max)).enumerate()
	.map(|(index, x)| cells[x as usize][y_orig as usize - len_origin_min as usize + index])
	.collect();

	let len_origin_min = (y_max - y_orig).min(x_orig - x_min);
	let len_origin_max = (y_orig - y_min).min(x_max - x_orig);
	let diag2: Vec<Stone> = ((x_orig-len_origin_min)..=(x_orig + len_origin_max)).enumerate()
	.map(|(index, x)| cells[x as usize][y_orig as usize + len_origin_min as usize - index])
	.collect();

	// print_slice(&horizontal);
	// print_slice(&vertical);
	// print_slice(&diag1);
	// print_slice(&diag2);

	let other_stone = match stone {
		Stone::WHITE => Stone::BLACK,
		Stone::BLACK => Stone::WHITE,
		_ => Stone::WHITE,
	};
	let list = [horizontal, vertical, diag1, diag2];
	for elem in &list {
		print_slice(&elem);
		value += eval_line(&elem, stone, &other_stone);
	}
	println!("VALUE TOTAL =====> {}", value);
	value
}

pub fn print_slice(slice: &[Stone]) {
	println!("------");
	for e in slice {
		match e {
			Stone::WHITE => print!("x "),
			Stone::BLACK => print!("o "),
			Stone::NOPE => print!(". "),
		}
	}
	println!("");
}

pub fn analyze_slice_of_6(slice: &[Stone], current_stone: &Stone, other_stone: &Stone) -> isize {

	println!("==> {:?}", slice);

	match slice {
		
		test if test[0] == *current_stone && test[0] == test[1] && test[0] == test[2] && test[0] == test[3] && test[0] == test[4] => 42,
		// [_, s0, s1, s2, s3, s4] if *s0 == current_stone && s0 == s1 && s0 == s2 && s0 == s3 && s0 == s4 => 42,
		// [s0, s1, s2, s3, s4, _] if *s0 == current_stone && s0 == s1 && s0 == s2 && s0 == s3 && s0 == s4 => 42,
		[Stone::NOPE, s1, s2, s3, s4, Stone::NOPE] => {
			print!("[Stone::NOPE, s1, s2, s3, s4, Stone::NOPE]!  ");
			match (s1,s2,s3,s4) {
				(s1, s2, s3, s4) if s1 == current_stone && s2 == other_stone && s2 == s3 && s1 == s4 => {
					print!("capture! ");
					2
				},				// capture
				(s1, s2, s3, s4) if s1 == current_stone && s1 == s2 && s1 == s3 && s1 == s4 => {
					print!("align 4! ");
					4
					},	// align 4
				(s1, s2, s3, Stone::NOPE) if s1 == current_stone && s1 == s2 && s1 == s3 => {
					print!("align 3_0  ");
					3
				},		// align 3
				(s1, s2, Stone::NOPE, s3) if s1 == current_stone && s1 == s2 && s1 == s3 => {
					print!("align 3_1  ");
					3
				},		// align 3
				(s1, Stone::NOPE, s2, s3) if s1 == current_stone && s1 == s2 && s1 == s3 => {
					print!("align 3_2  ");
					3
				},		// align 3
				(s1, s2, Stone::NOPE, Stone::NOPE) if s1 == current_stone && s1 == s2 => {
					print!("align 1  ");
					1
				},		// align 3

				// (Stone::NOPE, s1, s2, s3) if *s1 == *current_stone && s1 == s2 && s1 == s3 => {
				// 	print!("align 3_3  ");
				// 	3
				// },		// align 3
				_ => 0,
			}
		}
		// test if test[0] == *current_stone && test[0] == test[1] && test[0] == test[2] && test[0] == test[3] && test[0] == test[4] => 1,

		[Stone::NOPE, s1, s2, Stone::NOPE, Stone::NOPE, Stone::NOPE] if *s1 == *current_stone && s1 == s2 => 1,
		_ => 0,
	}
}

pub fn eval_line(slice: &[Stone], current_stone: &Stone, other_stone: &Stone) -> isize {

	let mut value = 0;
	let mut len = slice.len();
	// println!("eval_line, len: {}", len);
	if len < 5 { return 0; }

	while len > 6 {
		value += analyze_slice_of_6(&slice[len-6..len], current_stone, other_stone);
		println!("value: {}", value);
		len -= 1;
	}
	if len > 0 {
		value += analyze_slice_of_6(&slice[0..len], current_stone, other_stone);
		println!("value: {}", value);

	}
	println!("value of line: {}", value);


	value
}

impl Gameboard {

	// pub fn eval(&self) -> isize {
	// 	0
	// }

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
				if tmp_x < 0 || tmp_x >= self.size as isize || tmp_y < 0 || tmp_y >= self.size as isize {
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
		selected_move
	}
}

impl Gameboard {
	// pub fn apply_capture(&mut self) -> u8 {

	// }

	pub fn make_move(&mut self, x: usize, y: usize, stone: Stone) -> bool {
		if self.cells[x][y] == Stone::NOPE && !self.check_double_tree(x, y, stone) {
				// let nbr_capture = apply_capture();
			self.cells[x][y] = stone;
			self.update_possible_move(x, y);
			println!("\n-------------------");
			self.printboard();
			println!("x: {}, y: {}", x, y);
			self.value = eval_value(&self.cells, x as isize, y as isize, &stone);

                return true;
        }
        false
    }
	
	pub fn printboard(&self) {
		print!("BOARD: \n   ");
		for x in 0..SIZE {
			print!("{0: <2} ", x);
		}
		println!("");

		for y in 0..SIZE {
			print!("{0: <2} ", y);
			for x in 0..SIZE {
				match self.cells[x][y] {
					Stone::WHITE => print!("W  "),
					Stone::BLACK => print!("B  "),
					_ => print!(".  ")
				}
			}
			println!("");
		}
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
        // println!("nbr_tree = {}", nbr_tree);
        nbr_tree >= 2
	}
}

