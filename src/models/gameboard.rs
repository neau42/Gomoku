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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Alignement {
	pub alignement_type: AlignType,
	pub start_x: usize,
	pub start_y: usize,
	pub end_x: usize,
	pub end_y: usize ,
	pub before: Option<[usize; 2]>,
	pub after: Option<[usize; 2]>,
	// pub with_hole: bool,
}
impl Alignement {
	pub fn print_align(&self) {
		// println!("\tALIGNEMENT:");
		match self.alignement_type {
		AlignType::HORIZONTAL => {
			println!("\t- HORIZONTAL, (len:{})[{}][{}]<-->[{}][{}]", len_of_one_align(self), self.start_x, self.start_y, self.end_x, self.end_y);
		},
		AlignType::VERTICAL => {
			println!("\t- VERTICAL, (len:{})[{}][{}]<-->[{}][{}]",len_of_one_align(self), self.start_x, self.start_y, self.end_x, self.end_y);
		},
		AlignType::DIAGONAL1 => {
			println!("\t- DIAGONAL1, (len:{})[{}][{}]<-->[{}][{}]",len_of_one_align(self), self.start_x, self.start_y, self.end_x, self.end_y);
		},
		AlignType::DIAGONAL2 => {
			println!("\t- DIAGONAL2, (len:{})[{}][{}]<-->[{}][{}]",len_of_one_align(self), self.start_x, self.start_y, self.end_x, self.end_y);
		},
	}
	}

}

/// Stores game board information.
#[derive(Debug, Eq, Clone)]
pub struct Gameboard {
	pub size: usize,
    pub cells: [[Stone; SIZE]; SIZE],
    pub upperbound: isize,
    pub lowerbound: isize,
	pub align_list_black: Vec<Alignement>,
	pub align_list_white: Vec<Alignement>,
	pub win: [usize; 4], 
}

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

	pub fn set_stone_on_cell(&self, x: usize, y: usize, stone: Stone) -> Option<Gameboard> {
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

    // pub fn check_capture(&self, y: usize, x: usize, actual_stone: Stone) -> bool {
    //     let directions: [(isize, isize); 8] = [(0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1)];

    //     directions.iter().any(|(tmp_x, tmp_y)| {
    //         (1..3 as isize).all(|i| {
    //             let tmp_x = *tmp_x  * i + x as isize;
    //             let tmp_y = *tmp_y * i + y as isize;
    //             if tmp_x < 0 || tmp_y < 0 {
    //                 return false;
    //             }
    //             let tmp_stone = self.cells[tmp_x as usize][tmp_y as usize];
    //             match i {
    //                 1 | 2 => tmp_stone != actual_stone && tmp_stone != Stone::NOPE,
    //                 _ => tmp_stone == actual_stone,
    //             }
    //         })
    //     })
	// }
// }

    //Check si avec cette etats : On a le bon nombre d'element aligner ou de capture
    pub fn is_finish_state(&self) -> bool {
        false
    }

	// fn parse_arround_one(&self, line:[(isize, isize); 5], stone: Stone, one_hole: &mut bool) -> Vec<(usize, usize, bool)> {

	fn parse_arround_one(&self, line:[(isize, isize); 5], stone: Stone) -> (isize, Option<[usize;2]>) {
		// let mut one_hole = false;
		// let mut is_open: bool = false;
		// let open: [usize;2];
		// let mut open_or_hole:bool = *one_hole;
		let mut len = 0;

		for (x, y) in line.iter().filter(
			|(x, y)| *x >= 0
			&& *x < self.size as isize
			&& *y >= 0
			&& *y < self.size as isize) {

			if self.cells[*x as usize][*y as usize] == Stone::NOPE {
				// if open_or_hole == true {
					let open = [*x as usize, *y as usize];
					return (len, Some(open));
				}
				// open_or_hole = true;
			// }
			else if self.cells[*x as usize][*y as usize] == stone {
				// if open_or_hole == true {
				// 	len += 1;
				// 	*one_hole = true;
				// 	open_or_hole = false;
				// }
				len += 1;
			}
			else { break ; }
		}
		// if len > 0 {
		// 	println!("parse_around_one: LINE: {:?} {:?} {:?} {:?} {:?}", line[0], line[1], line[2], line[3], line[4]);
		// 	println!("RESULT: len: {}, is_open: {}, one_hole: {}", len, is_open, one_hole);
		// }
		(len, None)
	}

	pub fn print_all_align(&self) {
		let mut h = Vec::new();
		let mut v = Vec::new();
		let mut d1 = Vec::new();
		let mut d2 = Vec::new();

		println!("align WHITE: ");
		for align in &self.align_list_white {
			match align.alignement_type {
				AlignType::HORIZONTAL => h.push(align),
				AlignType::VERTICAL => v.push(align),
				AlignType::DIAGONAL1 => d1.push(align),
				AlignType::DIAGONAL2 => d2.push(align),
			}	
		}
		h.extend(v);
		h.extend(d1);
		h.extend(d2);
		for a in h {
			a.print_align();
		}
		let mut h = Vec::new();
		let mut v = Vec::new();
		let mut d1 = Vec::new();
		let mut d2 = Vec::new();

		println!("align BLACK: ");
		for align in &self.align_list_black {
			match align.alignement_type {
				AlignType::HORIZONTAL => h.push(align),
				AlignType::VERTICAL => v.push(align),
				AlignType::DIAGONAL1 => d1.push(align),
				AlignType::DIAGONAL2 => d2.push(align),
			}	
		}
		h.extend(v);
		h.extend(d1);
		h.extend(d2);
		for a in h {
			a.print_align();
		}
	}


	fn parse_around_cell(&self, align_type: AlignType, before:[(isize, isize); 5], after: [(isize, isize); 5] , x_orig: isize, y_orig: isize, stone: Stone) -> Option<Alignement> {
		// let mut with_hole = false;
		let (before_len, before) = self.parse_arround_one(before, stone);
		let (after_len, after) = self.parse_arround_one(after, stone);
		// let hole = if hole_before == true || hole_after == true
		// { true } else { false };

		if before_len > 0 || after_len > 0 {
			// println!("create new align");
			match align_type {
				AlignType::HORIZONTAL => {
					Some(Alignement {
					alignement_type: align_type,
					start_x: (x_orig - before_len) as usize,
					start_y: y_orig as usize,
					end_x: (x_orig + after_len) as usize,
					end_y: y_orig as usize,
					before,
					after,
					// with_hole
			})
				},
				AlignType::VERTICAL => {
				Some(Alignement {
					alignement_type: align_type,
					start_x: x_orig as usize,
					start_y: (y_orig - before_len) as usize,
					end_x: x_orig as usize,
					end_y: (y_orig + after_len) as usize,
					before,
					after,
					// with_hole
				})
				},
				AlignType::DIAGONAL1 => {
				Some(Alignement {
					alignement_type: align_type,
					start_x: (x_orig - before_len) as usize,
					start_y: (y_orig - before_len) as usize,
					end_x: (x_orig + after_len) as usize,
					end_y: (y_orig + after_len) as usize,
					before,
					after,
					// with_hole
			})
				},
				AlignType::DIAGONAL2 => {
				Some(Alignement {
					alignement_type: align_type,
					start_x: (x_orig - before_len) as usize,
					start_y: (y_orig + before_len) as usize,
					end_x: (x_orig + after_len) as usize,
					end_y: (y_orig - after_len) as usize,
					before,
					after,
					// with_hole
			})
				},
			}

		}
		else { None	}
	}

	pub fn new_aligns_h(&self, x_orig: isize, y_orig: isize, stone: Stone) -> Option<Alignement> {
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

		// println!("new_aligns_horizontal");
		self.parse_around_cell(AlignType::HORIZONTAL, before_horizontal, after_horizontal, x_orig, y_orig, stone)
	}
	pub fn new_aligns_v(&self, x_orig: isize, y_orig: isize, stone: Stone) -> Option<Alignement> {

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

		// println!("new_aligns_vertical");
		self.parse_around_cell(AlignType::VERTICAL, before_vertical, after_horizontal, x_orig, y_orig, stone)
	}
	pub fn new_aligns_d1(&self, x_orig: isize, y_orig: isize, stone: Stone) -> Option<Alignement> {

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
		// println!("new_aligns_diag 1");
		self.parse_around_cell(AlignType::DIAGONAL1, before_diag_1, after_diag_1, x_orig, y_orig, stone)
	}

	pub fn new_aligns_d2(&self, x_orig: isize, y_orig: isize, stone: Stone) -> Option<Alignement> {


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

		// println!("new_aligns_diag 2");
		self.parse_around_cell(AlignType::DIAGONAL2, before_diag_2, after_diag_2, x_orig, y_orig, stone)
	}


	
	pub fn update_list_black(&mut self, align: Alignement) {
		let index = self.align_list_black.iter().position(
			|e| (e.alignement_type == align.alignement_type
				&& ((e.start_x == align.start_x && e.start_y == align.start_y)
					|| (e.end_x == align.end_x && e.end_y == align.end_y))));
		let t;
		match index {
			Some(a) => {
				t = self.align_list_black.remove(a);
				// println!("REPLACE ALIGN:");
				// t.print_align();
				// println!("BY ALIGN:");
				// align.print_align();
			},
			_ => (),
		}
		self.align_list_black.push(align);
	}
		pub fn update_list_white(&mut self, align: Alignement) {
		let index = self.align_list_white.iter().position(
			|e| (e.alignement_type == align.alignement_type
				&& ((e.start_x == align.start_x && e.start_y == align.start_y)
					|| (e.end_x == align.end_x && e.end_y == align.end_y))));
		let t;
		match index {
			Some(a) => {
				t = self.align_list_white.remove(a);
				// println!("REPLACE ALIGN:");
				// t.print_align();
				// println!("BY ALIGN:");
				// align.print_align();
			},
			_ => (),
		}
		self.align_list_white.push(align);
	}

	pub fn set_align(&mut self, ref_gameboard: &Gameboard, x: usize, y: usize, stone: Stone) {

	// println!("--------------------\n\nset_align on:  ([{}][{}])", x, y);
	// self.printboard();

    static ALIGN_FUNCTIONS: &[fn(&Gameboard, x_orig: isize, y_orig: isize, stone: Stone) -> Option<Alignement>;
         4] = &[Gameboard::new_aligns_h, Gameboard::new_aligns_v,Gameboard::new_aligns_d1, Gameboard::new_aligns_d2];

		if stone == Stone::BLACK {
			self.align_list_black = ref_gameboard.align_list_black.clone();
			for fn_align in ALIGN_FUNCTIONS {
				let test = fn_align(&self, x as isize, y as isize, stone);
				match test {
					Some(t) => {
						self.update_list_black(t);
					}
					_ => (),
				}
				// println!("");
			}
		}
		else if stone == Stone::WHITE
		{
			self.align_list_white = ref_gameboard.align_list_white.clone();
			for align in ALIGN_FUNCTIONS {
				let test = align(&self, x as isize, y as isize, stone);
				match test {
					Some(t) => {
						// println!("\tALIGN::(WHITE)");
						// t.print_align();
						self.update_list_white(t);
						// self.align_list_white.push(t);
					}
					_ => (),
				}
				// println!("");

			}
		}
	}
	pub fn eval(&self) -> isize {
        0
    }

    pub fn expand(&self, stone: Stone) -> Vec<Gameboard> {

		let range_h:Vec<usize> = (self.win[0]..self.win[2] as usize).collect();
		let range_v:Vec<usize> = (self.win[1]..self.win[3] as usize).collect();
        let vector: Vec<Gameboard> = range_h.iter()
			 .map(|x| range_v
				.iter()
				.map(|y| self.set_stone_on_cell(*x, *y, stone))
				.filter_map(|state| state)
				.collect())
			 .collect::<Vec<Vec<Gameboard>>>()
			 .concat();
        vector
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

	pub fn victory(&self) -> bool {
		// for align in &self.align_list_white {
		// 	if align.with_hole == false && len_of_one_align(align) > 4 {
		// 		return true;
		// 	}
		// }
		// for align in &self.align_list_black {
		// 	if align.with_hole == false && len_of_one_align(align) > 4 {
		// 		return true;
		// 	}
		// }
	false
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
			self.win[2] = x + 2;
			// println!("new windows X max: {}", self.win[2]);
			if self.win[2] >= self.size {
				self.win[2] = self.size;
			}
		}
		if self.win[3] <= y {
			self.win[3] = y + 2;
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

pub fn len_of_one_align(align: &Alignement) -> usize {
	match align.alignement_type {
		AlignType::HORIZONTAL => (((align.end_x - align.start_x) as isize).abs() + 1) as usize,
		AlignType::VERTICAL => (((align.end_y - align.start_y) as isize).abs() + 1) as usize,
		AlignType::DIAGONAL1 => (((align.end_y - align.start_y) as isize).abs() + 1) as usize,
		AlignType::DIAGONAL2 => (((align.end_x - align.start_x) as isize).abs() + 1) as usize,
	}
}