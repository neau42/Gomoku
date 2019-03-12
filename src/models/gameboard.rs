use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Size of game board.
pub const SIZE: usize = 19;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Stone {
    BLACK,
    WHITE,
	NOPE,
}

/// Stores game board information.
#[derive(Debug, Eq, Clone, Copy)]
pub struct Gameboard {
	pub size: usize,
    pub cells: [[Stone; SIZE]; SIZE],
	// pub white_stone: [[bool; SIZE]; SIZE],
	// pub black_stone: [[bool; SIZE]; SIZE],
    pub upperbound: isize,
    pub lowerbound: isize,
	///pub win: windows arround actives cells (x_start, y_start, x_end, y_end)
	pub win: [usize; 4], 
}

/// Creates a new game board.
impl Gameboard {
	pub fn new() -> Gameboard {
		Gameboard {
			size: SIZE,
			cells: [[Stone::NOPE; SIZE]; SIZE],
		
			// white_stone: [[false; SIZE]; SIZE],
			// black_stone: [[false; SIZE]; SIZE],

            upperbound: isize::from(std::i16::MAX),
            lowerbound: isize::from(std::i16::MIN),
			win: [0, 0, SIZE, SIZE],
		}
	}
}

impl Gameboard {
    pub fn set_stone_on_cell(&self, y: usize, x: usize, stone: Stone) -> Option<Gameboard> {
			// println!("set_stone_on_cell: x:{} y:{}", x, y);

		if self.cells[x][y] == Stone::NOPE {
            let mut new_state = self.clone();
			new_state.cells[x][y] = stone;
			new_state.set_window_actives_cells();
			
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

    pub fn eval(&self) -> isize {
        0
    }

    pub fn expand(&self, stone: Stone) -> Vec<Gameboard> {

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

	pub fn set_window_actives_cells(&mut self) {
	let mut first_x = self.size as isize;
	let mut first_y = self.size as isize;
	let mut last_x = 0 as isize;
	let mut last_y = 0 as isize;

	for y in 0..SIZE as isize {
		for x in 0..SIZE as isize {
			match self.cells[x as usize][y as usize] {
				Stone::NOPE => (),
				_ => {
					if x > last_x { last_x = x; }
					if y > last_y { last_y = y; }
					if x < first_x { first_x = x; }
					if y < first_y { first_y = y; }
					},
	} } }
	first_x -= 1;
	first_y -= 1;
	last_x += 2;
	last_y += 2;
	if first_x < 0 { first_x = 0};
	if first_y < 0 { first_y = 0};
	if last_x >= self.size as isize { last_x = self.size as isize};
	if last_y >= self.size as isize { last_y = self.size as isize};
	// println!("get_window_actives_cells: first_x: {} , first_y: {} , last_x: {} , last_y: {}", first_x, first_y, last_x, last_y);
	self.win[0] = first_x as usize;
	self.win[1] = first_y as usize;
	self.win[2] = last_x as usize;
	self.win[3] = last_y as usize;
	// [first_x as usize , first_y as usize , last_x as usize , last_y as usize]
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