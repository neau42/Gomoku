//! Gameboard and info about actual state
//! coucou

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

#[derive(Debug,PartialEq, Eq, Copy, Clone)]
pub enum AlignType {
    HORIZONTAL,
    VERTICAL,
	DIAGONAL1,
	DIAGONAL2,
}

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub struct Alignement {
// 	pub alignemnt_type: AlignType,
// 	pub start_x: usize,
// 	pub start_y: usize,
// 	pub end_x: usize,
// 	pub end_y: usize ,
// 	pub before_open: bool,
// 	pub after_open: bool,
// }

/// Stores game board information.
#[derive(Debug, Eq, Clone)]
pub struct Gameboard {
	pub size: usize,
    pub cells: [[Stone; SIZE]; SIZE],
    pub upperbound: isize,
    pub lowerbound: isize,
	pub align_list_black: Vec<(AlignType, usize, usize, usize, usize , bool, bool)>,
	pub align_list_white: Vec<(AlignType, usize, usize, usize, usize , bool, bool)>,
	pub win: [usize; 4], 
}

// impl Copy for Gameboard { }

// impl Clone for Gameboard {
//     fn clone(&self) -> Gameboard {
//         *self
//     }
// }

/// Creates a new game board.
impl Gameboard {
	pub fn new() -> Gameboard {
		Gameboard {
			size: SIZE,
			cells: [[Stone::NOPE; SIZE]; SIZE],
            upperbound: isize::from(std::i16::MAX),
            lowerbound: isize::from(std::i16::MIN),
			align_list_black: Vec::new(),
			align_list_white: Vec::new(),
			win: [SIZE, SIZE, 0, 0],
		}
	}
}


// pub fn len_of_one_align(align: &(AlignType, usize, usize, usize, usize , bool, bool)) -> usize {

// 	match align.0 {
// 		AlignType::HORIZONTAL => align.3 - align.1,
// 		AlignType::VERTICAL => align.4 - align.2,
// 		AlignType::DIAGONAL1 => align.3 - align.1,
// 		AlignType::DIAGONAL2 => align.3 - align.1,
// 	}
// }

impl Gameboard {

	pub fn set_stone_on_cell(&self, y: usize, x: usize, stone: Stone) -> Option<Gameboard> {
		if self.cells[x][y] == Stone::NOPE {
			let mut new_state = self.clone();
			new_state.cells[x][y] = stone;
			new_state.set_window_actives_cells(x, y);
			// println!("NEW STATE: WINDOW: Xmin:{} Ymin:{}, Xmax:{}, Ymax:{} ", new_state.win[0], new_state.win[1], new_state.win[2], new_state.win[3]);
			new_state.set_align(self, x, y, stone);
            Some(new_state)
		} else {
			None
		}
	}

    pub fn check_capture(&self, y: usize, x: usize, actual_stone: Stone) -> bool {
        let directions: [(isize, isize); 8] = [(0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1)];

        directions.iter().any(|(tmp_x, tmp_y)| {
            (1..3 as isize).all(|i| {
                let tmp_x = *tmp_x  * i + x as isize;
                let tmp_y = *tmp_y * i + y as isize;
                if tmp_x < 0 || tmp_y < 0 {
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
}

impl Gameboard {

    //Check si avec cette etats : On a le bon nombre d'element aligner ou de capture
    pub fn is_finish_state(&self) -> bool {
        false
    }


	fn parse_arround_one(&self, line:[(isize, isize); 5], stone: Stone) -> (isize, bool, bool) {
		let mut one_empty = false;
		let mut is_open = false;
		let mut len = 0;

		for (x, y) in line.iter().filter(
			|(x, _y)| *x >= 0 as isize 
			&& *x < self.size as isize) {

			if self.cells[*x as usize][*y as usize] == Stone::NOPE {
				if one_empty == true {
					is_open = true;
					break ;
				}
				one_empty = true;
			}
			else if self.cells[*x as usize][*y as usize] == stone {
				len += 1;
			}
			else { break ; }
		}
		(len, is_open, one_empty)
	}

	fn parse_around_cell(&self, align_type: AlignType, before_horizontal:[(isize, isize); 5], after_horizontal: [(isize, isize); 5] , x_orig: isize, y_orig: isize, stone: Stone) -> Option<(AlignType, usize, usize, usize, usize , bool, bool)> {
		let (before_len, open_before, hole_before) = self.parse_arround_one(before_horizontal, stone);
		let (after_len, open_after, hole_after) = self.parse_arround_one(after_horizontal, stone);

		if before_len > 0 || after_len > 0 {
			Some((align_type, (x_orig - before_len) as usize, y_orig as usize, (x_orig + after_len) as usize, y_orig as usize, open_before, open_after))
		}
		else { None	}
	}


	pub fn new_aligns_h(&self, x_orig: isize, y_orig: isize, stone: Stone) -> Option<(AlignType, usize, usize, usize, usize , bool, bool)> {
		let before_horizontal: [(isize, isize); 5] = [
			(x_orig - 1, y_orig),
			(x_orig - 2, y_orig),
			(x_orig - 3, y_orig),
			(x_orig - 4, y_orig),
			(x_orig - 5, y_orig)];
		let after_horizontal: [(isize, isize); 5] = [
			(x_orig + 1, y_orig),
			(x_orig + 2, y_orig),
			(x_orig + 3, y_orig),
			(x_orig + 4, y_orig),
			(x_orig + 5, y_orig)];

		self.parse_around_cell(AlignType::HORIZONTAL, before_horizontal, after_horizontal, x_orig, y_orig, stone)
	}
	pub fn new_aligns_v(&self, x_orig: isize, y_orig: isize, stone: Stone) -> Option<(AlignType, usize, usize, usize, usize , bool, bool)> {

		let before_vertical: [(isize, isize); 5] = [
			(x_orig, y_orig - 1),
			(x_orig, y_orig - 2),
			(x_orig, y_orig - 3),
			(x_orig, y_orig - 4),
			(x_orig, y_orig - 5)];
		let after_horizontal: [(isize, isize); 5] = [
			(x_orig, y_orig + 1),
			(x_orig, y_orig + 2),
			(x_orig, y_orig + 3),
			(x_orig, y_orig + 4),
			(x_orig, y_orig + 5)];

		self.parse_around_cell(AlignType::VERTICAL, before_vertical, after_horizontal, x_orig, y_orig, stone)
	}
	pub fn new_aligns_d1(&self, x_orig: isize, y_orig: isize, stone: Stone) -> Option<(AlignType, usize, usize, usize, usize , bool, bool)> {

		let before_diag_1: [(isize, isize); 5] = [
			(x_orig - 1, y_orig - 1),
			(x_orig - 2, y_orig - 2),
			(x_orig - 3, y_orig - 3),
			(x_orig - 4, y_orig - 4),
			(x_orig - 5, y_orig - 5)];
		let after_diag_1: [(isize, isize); 5] = [
			(x_orig + 1, y_orig + 1),
			(x_orig + 2, y_orig + 2),
			(x_orig + 3, y_orig + 3),
			(x_orig + 4, y_orig + 4),
			(x_orig + 5, y_orig + 5)];

		self.parse_around_cell(AlignType::DIAGONAL1, before_diag_1, after_diag_1, x_orig, y_orig, stone)
	}

	pub fn new_aligns_d2(&self, x_orig: isize, y_orig: isize, stone: Stone) -> Option<(AlignType, usize, usize, usize, usize , bool, bool)> {


		let before_diag_2: [(isize, isize); 5] = [
					(x_orig - 1, y_orig + 1),
					(x_orig - 2, y_orig + 2),
					(x_orig - 3, y_orig + 3),
					(x_orig - 4, y_orig + 4),
					(x_orig - 5, y_orig + 5)];
		let after_diag_2: [(isize, isize); 5] = [
					(x_orig + 1, y_orig - 1),
					(x_orig + 2, y_orig - 2),
					(x_orig + 3, y_orig - 3),
					(x_orig + 4, y_orig - 4),
					(x_orig + 5, y_orig - 5)];

		self.parse_around_cell(AlignType::DIAGONAL2, before_diag_2, after_diag_2, x_orig, y_orig, stone)
	}

				// let diag_2: [(isize, isize); 10] = [
			// }
			// }

				// let tmp = afters.iter().map(|after| self.get_better_align_on_one_cell(x, y, *after)).max();
				// match tmp {
				// 	Some(test) => { if test > value {value = test;}}
				// 	_ => (),
				// }

	pub fn set_align(&mut self, ref_gameboard: &Gameboard, x: usize, y: usize, stone: Stone) {

    static ALIGN_FUNCTIONS: &[fn(&Gameboard, x_orig: isize, y_orig: isize, stone: Stone) -> Option<(AlignType, usize, usize, usize, usize , bool, bool)>;
         4] = &[Gameboard::new_aligns_h, Gameboard::new_aligns_v,Gameboard::new_aligns_d1, Gameboard::new_aligns_d2];

		if stone == Stone::BLACK {
			self.align_list_black = ref_gameboard.align_list_black.clone();
			for align in ALIGN_FUNCTIONS {
				let test = align(&self, x as isize, y as isize, stone);
				match test {
					Some(t) => {
						self.printboard();
						self.align_list_black.push(t);
					}
					_ => (),
				}
			}
		}
		else if stone == Stone::WHITE
		{
			self.align_list_white = ref_gameboard.align_list_white.clone();
			for align in ALIGN_FUNCTIONS {
				let test = align(&self, x as isize, y as isize, stone);
				match test {
					Some(t) => {
						self.printboard();
						self.align_list_white.push(t);
					}
					_ => (),
				}
			}
		}
	}
	pub fn eval(&self) -> isize {
        0
    }

    pub fn expand(&self, stone: Stone) -> Vec<Gameboard> {

		println!("expand");
        // let range: Vec<usize> = (0..SIZE as usize).collect();
        // let vector: Vec<Gameboard> = range.iter()
			//  .map(|y| range
		let range_h:Vec<usize> = (self.win[0]..self.win[2] as usize).collect();
		let range_v:Vec<usize> = (self.win[1]..self.win[3] as usize).collect();
        let vector: Vec<Gameboard> = range_v.iter()
			 .map(|y| range_h
				.iter()
				.map(|x| self.set_stone_on_cell(*y, *x, stone))
				.filter_map(|state| state)
				// .map(|state| state.set_align(self))
				.collect())
			 .collect::<Vec<Vec<Gameboard>>>()
			 .concat();
        // println!("len = {}", vector.len());
        vector
    }

	pub fn printboard(&self) {
		println!("BOARD: ");
		for y in 0..SIZE {
			for x in 0..SIZE {
				match self.cells[x][y] {
					Stone::WHITE => print!("W "),
					Stone::BLACK => print!("B "),
					_ => print!(". ")
				}
			}
			println!("");
		}
	}

pub fn get_better_align_on_one_cell(&self, x_orig: isize, y_orig: isize, after: [(isize, isize); 4]) -> isize {

	let first_stone = self.cells[x_orig as usize][y_orig as usize];
	if first_stone == Stone::NOPE { return 0;}

	let mut align_len = 1;
	for (x, y) in after.iter().filter( |(x, y)|
	*x >= self.win[0] as isize
	&& *y >= self.win[1] as isize
	&& *x < self.win[2] as isize
	&& *y < self.win[3] as isize) {
		let current_stone = self.cells[*x as usize][*y as usize];
		if current_stone == first_stone {
			align_len += 1;
		}
		else {break ;}
	}
	align_len
}

	// test get victory!
	pub fn max_align(&self) -> usize {
		let mut value = 0;

		for y in self.win[1] as isize..self.win[3] as isize {
			for x in self.win[0] as isize..self.win[2] as isize {
				let after_horizontal: [(isize, isize); 4] = [(x + 1, y),
					(x + 2, y),
					(x + 3, y),
					(x + 4, y)];
				let after_vertical: [(isize, isize); 4] = [(x, y + 1),
					(x, y + 2),
					(x, y + 3),
					(x, y + 4)];
				let after_diag_1: [(isize, isize); 4] = [(x + 1, y + 1),
					(x + 2, y + 2),
					(x + 3, y + 3),
					(x + 4, y + 4)];
				let after_diag_2: [(isize, isize); 4] = [(x + 1, y - 1),
					(x + 2, y - 2),
					(x + 3, y - 3),
					(x + 4, y - 4)];
				let afters = [after_horizontal, after_vertical, after_diag_1, after_diag_2];

				let tmp = afters.iter().map(|after| self.get_better_align_on_one_cell(x, y, *after)).max();
				match tmp {
					Some(test) => { if test > value {value = test;}}
					_ => (),
				}

		}}
		if value > 4 {
			println!("WIN\n\n\n WIN\n\nmax ALIGN : return {}", value);
			self.printboard();
		}
	value as usize
	}

	pub fn set_window_actives_cells(&mut self, x: usize, y: usize) {
		if self.win[0] >= x {
			self.win[0] = x - 1;
			// println!("new windows X min: {}", self.win[0]);
			if self.win[0] >= self.size {
				self.win[0] = 0;
			}
		}
		if self.win[1] >= y {
			self.win[1] = y - 1;
			// println!("new windows Y min: {}", self.win[1]);
			if self.win[1] >= self.size {
				self.win[1] = 0;
			}
		}
		if self.win[2] <= x {
			self.win[2] = x + 1;
			// println!("new windows X max: {}", self.win[2]);
			if self.win[2] >= self.size {
				self.win[2] = self.size;
			}
		}
		if self.win[3] <= y {
			self.win[3] = y + 1;
			// println!("new windows Y max: {}", self.win[3]);
			if self.win[3] >= self.size {
				self.win[3] = self.size;
			}
		}
	}
}

impl PartialOrd for Gameboard {
    fn partial_cmp(&self, other: &Gameboard) -> Option<Ordering> {
        other.upperbound.partial_cmp(&self.upperbound)//To change
    }
}

impl PartialEq for Gameboard {
    fn eq(&self, other: &Gameboard) -> bool {
        self.cells == other.cells
    }
}

impl Hash for Gameboard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cells.hash(state);
    }
}