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
    pub fn set_stone_on_cell(&self, x: usize, y: usize, stone: Stone) -> Option<Gameboard> {
		if self.cells[x][y] == Stone::NOPE {
            if self.check_double_tree(x, y, stone) {
                println!("you did a double tree");
                None
            }
            else {
                let mut new_state = self.clone();
			    new_state.cells[x][y] = stone;
                Some(new_state)
            }
		} else {
			None
		}
	}

    // True if capture is possible
    pub fn check_capture(&self, x: usize, y: usize, actual_stone: Stone) -> bool {
        let directions: [(isize, isize); 8] = [(0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1)];

        directions.iter().any(|(tmp_x, tmp_y)| {
            (1..=3 as isize).all(|i| {
                let tmp_x = *tmp_x  * i + x as isize;
                let tmp_y = *tmp_y * i + y as isize;
                if tmp_x < 0 || tmp_x >= self.size as isize || tmp_y < 0 || tmp_y >= self.size as isize {// ou superieur a size
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

    //True is double tree
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
                    match stone {
                        Stone::NOPE => sum + 2,
                        actual_stone => sum + 1,
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
        nbr_tree >= 2
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
        let vector: Vec<Gameboard>= range.iter().flat_map(|y| range.iter().map(move |x| self.set_stone_on_cell(*x, *y, stone)).filter_map(|state| state)).collect();
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