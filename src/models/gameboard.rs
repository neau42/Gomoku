use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Size of game board.
const SIZE: usize = 19;

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
    pub upperbound: isize,
    pub lowerbound: isize,
}

/// Creates a new game board.
impl Gameboard {
	pub fn new() -> Gameboard {
		Gameboard {
			size: SIZE,
			cells: [[Stone::NOPE; SIZE]; SIZE],
            upperbound: isize::from(std::i16::MAX),
            lowerbound: isize::from(std::i16::MIN),
		}
	}
}

impl Gameboard {
    pub fn set_stone_on_cell(&self, y: usize, x: usize, stone: Stone) -> Option<Gameboard> {
		if self.cells[x][y] == Stone::NOPE {
            let mut new_state = self.clone();
			new_state.cells[x][y] = stone;
            Some(new_state)
		} else {
			None
		}
	}

    pub fn check_capture(&self, y: usize, x: usize, actual_stone: Stone) -> bool {
        let directions: [(isize, isize); 8] = [(0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1)];

        directions.iter().any(|(x, y)| {
            (1..3 as isize).all(|i| {
                let tmp_x = *x  * i;
                let tmp_y = *y * i;
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
        let range: Vec<usize> = (0..SIZE as usize).collect();
        let vector: Vec<Gameboard>= range.iter().map(|y| range.iter().map(|x| self.set_stone_on_cell(*y, *x, stone)).filter_map(|state| state).collect()).collect::<Vec<Vec<Gameboard>>>().concat();
        // println!("len = {}", vector.len());
        vector
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