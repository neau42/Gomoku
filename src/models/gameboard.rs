/// Size of game board.
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
	pub fn count_captures_and_trees(&mut self, list: [u32; 4], x_orig: usize, y_orig: usize, stone: u8) -> (u8, u8) {
		let capture_form: u8 = get_capture_form!(stone);
		let tree_forms: [u16; 4] = get_tree_forms!(stone);
		list.iter().fold((0, 0), |capture_tree, line| {
			let mut nbr_capture_to_add = 0;
			let mut nbr_tree_to_add = 0;
			(0..7).for_each(|range| {
				let line_to_check: u32 = (line >> (range * 2));
				if line_to_check == 0 {
					return;
				}
				if concat_stones!(line_to_check, 4) as u8 == capture_form {
					nbr_capture_to_add += 1;
				}
				if tree_forms.contains(&(concat_stones!(line_to_check, 6) as u16)) {
					nbr_tree_to_add += 1;
				}
			});
			(capture_tree.0 + nbr_capture_to_add, capture_tree.1 + nbr_tree_to_add)
		})
	}

	pub fn try_make_move(&mut self, x: isize, y: isize, stone: u8) -> bool {
		let x_min = (x - 5).max(0) as usize;
		let x_max = (x + 5).min(SIZE as isize - 1) as usize;
		let y_min = (y - 5).max(0) as usize;
		let y_max = (y + 5).min(SIZE as isize - 1) as usize;

		println!("try_make_move");

		self.cells[x as usize] |= set_stone!(y, stone);
		let horizontal: u32 = line_horizontal!(self.cells, x_min, x_max, y as usize);
		let vertical: u32 = line_vertical!(self.cells[x as usize], y_min, y_max);
		let down_diago: u32 = down_diago!(self.cells, x as usize, x_min, x_max, y as usize, y_min, y_max);
		let up_diago: u32 = up_diago!(self.cells, x as usize, x_min, x_max, y as usize, y_min, y_max);
		
		let list: [u32; 4] = [horizontal, vertical, down_diago, up_diago];
		let (nb_captures, nb_trees) = self.count_captures_and_trees(list, x as usize, y as usize, stone);
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
				return true;
			}
			self.cells[x] &= clear_stone!(y);
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
