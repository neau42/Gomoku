/// Size of game board.
pub const SIZE: usize = 19;

pub const NOPE: u8 = 0b00;
pub const BLACK: u8 = 0b01;
pub const WHITE: u8 = 0b10;

macro_rules! get_stone {
	($line: expr, $y: expr) => {
		($line >> ($y * 2) & 0b11) as u8
	};
}

macro_rules! clear_stone {
	($y: expr) => {
		 !(11 << ($y * 2))
	};
}

macro_rules! set_stone {
	($y: expr, $stone: expr) => {
		($stone as u64) << ($y * 2)
	};
}

macro_rules! set_move {
	($y: expr) => {
		0b1 << $y
	};
}

macro_rules! opposite_stone {
	($stone: expr) => {
		!$stone & 0b11
	};
}
macro_rules! eval_raw {
	($raw: expr, $stone: expr) => {
		{
			if $raw == 0 {
				return 0;
			}
			// let mut i = 0;
			// let mut sum = 0;
			// while i < SIZE * 2 {
			// 	if (($raw >> i) & 0b11) {

			// }
			// }
				println!("eval_raw: {:#064b}", $raw);
			1
		}
	}
}

macro_rules! eval {
	($cells: expr, $stone: expr) => {
		(0..SIZE).map(|x| eval_raw!($cells[x], stone)).sum()
	}
}

macro_rules! printboard {
	($cells: expr) => {
		print!("BOARD:\n   ");
		for x in 0..SIZE { print!("{0: <2} ", x) };
		println!();

		for y in 0..SIZE {
			print!("{0: <2} ", y);
			for x in 0..SIZE {
				match get_stone!($cells[x], y) {
					WHITE => print!("W  "),
					BLACK => print!("B  "),
					_ => print!(".  ")
				}
			}
			println!();
		}
	};
}

macro_rules! line_horizontal {
	($cells: expr, $x_min: expr, $x_max: expr, $y: expr) => {
		($x_min..=$x_max).enumerate().fold(0, |value, (index, x)| {
			value | ((get_stone!($cells[x], $y) as u32) << (index * 2))
		})
	};
}

macro_rules! line_vertical {
	($line: expr, $y_min: expr, $y_max: expr) => {
		($line >> ($y_min * 2)) & ((1 << $y_max * 2 - 1) - 1)
	};
}

macro_rules! up_diago {
	($len_origin_min: expr, $len_origin_max: expr, $cells: expr, $x_orig: expr, $x_min: expr, $x_max: expr, $y_orig: expr, $y_min: expr, $y_max: expr) => {
		(($x_orig - $len_origin_min)..=($x_orig + $len_origin_max))
		.enumerate()
		.fold(0, |value, (index, x)| {
			value | ((get_stone!($cells[x], $y_orig - $len_origin_min + index) as u32) << (index * 2))
		})
	};

	($cells: expr, $x_orig: expr, $x_min: expr, $x_max: expr, $y_orig: expr, $y_min: expr, $y_max: expr) => {
		up_diago!(
			($y_orig - $y_min).min($x_orig - $x_min),
			($y_max - $y_orig).min($x_max - $x_orig),
			$cells, $x_orig, $x_min, $x_max, $y_orig, $y_min, $y_max)
	};
}

macro_rules! down_diago {
	($len_origin_min: expr, $len_origin_max: expr, $cells: expr, $x_orig: expr, $x_min: expr, $x_max: expr, $y_orig: expr, $y_min: expr, $y_max: expr) => { 
		(($x_orig - $len_origin_min)..=($x_orig + $len_origin_max))
			.enumerate()
			.fold(0, |value , (index, x)| {
				value | ((get_stone!($cells[x], $y_orig + $len_origin_min - index) as u32) << (index * 2))
			})
	};

	($cells: expr, $x_orig: expr, $x_min: expr, $x_max: expr, $y_orig: expr, $y_min: expr, $y_max: expr) => {
		down_diago!(
			($y_max - $y_orig).min($x_orig - $x_min),
			($y_orig - $y_min).min($x_max - $x_orig),
			$cells, $x_orig, $x_min, $x_max, $y_orig, $y_min, $y_max)
	};
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Gameboard {
    pub cells: [u64; SIZE],
	pub possible_moves: [u32; SIZE],
    pub selected_move: Option<(usize, usize)>,
	pub black_captures: u8,
	pub white_captures: u8,
}

impl Gameboard {
	pub fn new() -> Gameboard {
		Gameboard {
			cells: [0; SIZE],
			possible_moves: [0; SIZE],
            selected_move: None,
			black_captures: 0,
			white_captures: 0,
		}
	}

	pub fn is_final(&self) -> bool {
		self.black_captures == 5
		|| self.white_captures == 5
		// ou alignment 5 sans capture possible
    }
}

impl Gameboard {
	pub fn count_captures_and_trees(&mut self, list: [u32; 4], stone: u8) -> (u8, u8) {
		(0, 0)
	}

	pub fn try_make_move(&mut self, x: isize, y: isize, stone: u8) -> bool {
		let x_min = (x - 5).max(0) as usize;
		let x_max = (x + 5).min(SIZE as isize - 1) as usize;
		let y_min = (y - 5).max(0) as usize;
		let y_max = (y + 5).min(SIZE as isize - 1) as usize;

		println!("try_make_move [{}] [{}]", x, y);
		let horizontal: u32 = line_horizontal!(self.cells, x_min, x_max, y as usize);
		let vertical: u32 = line_vertical!(self.cells[x as usize] as u32, y_min as u32, y_max as u32);
		let down_diago: u32 = down_diago!(self.cells, x as usize, x_min, x_max, y as usize, y_min, y_max);
		let up_diago: u32 = up_diago!(self.cells, x as usize, x_min, x_max, y as usize, y_min, y_max);
		
		let list = [horizontal, vertical, down_diago, up_diago];
			println!("horizontal: {:#024b}", horizontal);
			println!("vertical  : {:#024b}", vertical);
			println!("down_diago: {:#024b}", down_diago);
			println!("up_diago  : {:#024b}", up_diago);
			printboard!(self.cells);

		let (nb_captures, nb_trees) = self.count_captures_and_trees(list, stone);
		if nb_captures > 0 {
			match stone {
				BLACK => self.black_captures += nb_captures,
				_ => self.white_captures += nb_captures
			}
		}
		nb_captures > 0 || nb_trees < 2
	}

	pub fn make_move(&mut self, x: usize, y: usize, stone: u8) -> bool {
		if get_stone!(self.cells[x], y) == NOPE {
			if self.try_make_move(x as isize, y as isize, stone) {
				self.cells[x] |= set_stone!(y, stone);
				return true;
			}
        }
        false
    }

	pub fn unmake_move(&mut self, x: usize, y: usize) {
        self.cells[x] &= clear_stone!(y);
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

	pub fn next_move(&mut self, mut starting_x: usize, mut starting_y: usize) {
        if starting_x >= SIZE {
            starting_x = 0;
            starting_y = starting_y + 1;
            if starting_y >= SIZE {
                self.selected_move = None;
                return;
            }
        }
        self.selected_move = None;
		(0..SIZE)
			.filter(|y| *y >= starting_y)
			.any(|y| (0..SIZE)
				.filter(|x| y > starting_y || *x >= starting_x)
				.any(|x| {
					let shift = y * 2;
					if self.possible_moves[x] >> y & 0b1 == 1 && (self.cells[x] >> shift) & 0b11 == 0 {
                        self.selected_move = Some((x, y));
						return true;
					}
					false
				})
        );
	}
}

impl Gameboard {
}