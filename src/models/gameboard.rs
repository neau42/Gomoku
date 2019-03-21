/// Size of game board.
pub const SIZE: usize = 19;
pub const NOPE: u8 = 0b00;
pub const BLACK: u8 = 0b01;
pub const WHITE: u8 = 0b10;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Gameboard {
	pub size: usize,
    pub cells: [u64; SIZE],
	pub possible_moves: [u32; SIZE],
    pub selected_move: Option<(usize, usize)>,
}

impl Gameboard {
	pub fn new() -> Gameboard {
		Gameboard {
			size: SIZE,
			cells: [0; SIZE],
			possible_moves: [0; SIZE],
            selected_move: None,
		}
	}
}

impl Gameboard {
	pub fn eval(&self) -> isize {
		0
	}

    pub fn make_move(&mut self, x: usize, y: usize, stone: u8) -> bool {
        let shift = y * 2;
        if (self.cells[x] >> shift) & 0b11 == 0  {
			    self.update_possible_move(x, y);
				self.cells[x] |= (stone as u64) << shift;
                return true;
        }
        false
    }

	pub fn unmake_move(&mut self, x: usize, y: usize) {
		let shift = y * 2;
        self.cells[x] = self.cells[x] & !(11 << shift);
    }
	
	pub fn update_possible_move(&mut self, x: usize, y: usize) {
        let directions: [(isize, isize); 8] = [(0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1)];
        directions.iter().for_each(|(tmp_x, tmp_y)| {
			let tmp_x = *tmp_x + x as isize;
			let tmp_y = *tmp_y + y as isize;
			if tmp_x < 0 || tmp_x >= self.size as isize || tmp_y < 0 || tmp_y >= self.size as isize {
				return;
			}
			if self.cells[tmp_x as usize] >> tmp_y & 0b1 == 0 {
				self.possible_moves[tmp_x as usize] |= 0b1 << tmp_y;
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