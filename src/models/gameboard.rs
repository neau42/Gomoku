use std::collections::HashSet;
use std::cmp::min;
use std::cmp::max;
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
#[derive(Debug, Eq)]
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
            upperbound: std::isize::MAX,
            lowerbound: std::isize::MIN,
		}
	}

	pub fn set_stone_on_cell(&mut self, y: usize, x: usize, stone: Stone) -> bool {
		if self.cells[x][y] == Stone::NOPE {
			self.cells[x][y] = stone;
			true
		} else {
			false
		}
	}
}


impl Gameboard {

    //Check si avec cette etats : On a le bon nombre d'element aligner ou de capture
    pub fn is_finish_state(&self) -> bool {
        true
    }

    pub fn eval(&self) -> isize {
        0
    }

    pub fn expand(&self) -> Vec<Gameboard> {
        Vec::new()
    }

    pub fn alpha_beta_with_memory(&mut self, all_state: &HashSet<Gameboard>, depth: usize, mut alpha: isize, mut beta: isize) -> isize {
        if all_state.contains(self) {
            if self.lowerbound >= beta {
                return self.lowerbound;
            }
            if self.upperbound <= alpha {
                return self.upperbound;
            }
            alpha = max(alpha, self.lowerbound);
            beta = min(beta, self.upperbound);
        }
        if self.is_finish_state() || depth <= 0 {
            return /*winning score or*/ self.eval();
        }
        // move bestmove ;
        let mut current = std::isize::MIN;
        //for (each possible move m
        for mut new_state in self.expand() {
            // make move m;
            let score = - new_state.alpha_beta_with_memory(all_state, depth - 1, -beta, -alpha); 
            // unmake move m;
            if score >= current {
                current = score;
                // bestmove = m;
                if score >= alpha {
                    alpha = score;
                    if score >= beta {
                        break;
                    }
                }
            }
        }
        if current <= alpha {
            self.upperbound = current;
        }
        if current >= beta {
            self.lowerbound = current;
        }
        return current;
    }

    pub fn mdtf(&mut self, all_state: HashSet<Gameboard>, mut g: isize, depth: usize) -> isize { //On utilise donc en général comme valeur de f la valeur retourné par l’algorithme lors d’une itération précédente
        let mut upperbound = std::isize::MAX;
        let mut lowerbound = std::isize::MIN;

        while lowerbound != upperbound {
            let beta: isize = match lowerbound {
                g => g + 1,
                _ => g,
            };
            g = self.alpha_beta_with_memory(&all_state, depth, beta - 1, beta);
            if g < beta {
                upperbound = g;
            }
            else {
                lowerbound = g;
            }
        }
        g
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