use crate::models::gameboard::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IA {
    pub depth: u8,
}

impl IA {
    pub fn new(depth: u8) -> IA {
        IA {
            depth,
        }
    }
}

impl IA {
    //Check si avec cette etats : On a le bon nombre d'element aligner ou de capture
    pub fn is_victory(&self) -> bool {
        false
    }

    pub fn eval(&self, gameboard: &Gameboard) -> isize {
		// println!("\n\n_____________");
		// gameboard.printboard();
		// println!("eval: {}", gameboard.value);

		-gameboard.value
    }

    /// si alpha < current < beta, alors current est la valeur minimax
    /// si current <= alpha, alors la vraie valeur minimax m vérifie : m <= current <= alpha
    /// si beta <= current alors la vraie valeur minimax m vérifie : beta <= current <= m

    pub fn negascout(&self, state: &mut Gameboard, stone: Stone, depth: u8, mut alpha: isize, beta: isize) -> isize {
        if depth == 0 || self.is_victory() {
            return self.eval(state);
        }
        let mut best_move: Option<(usize, usize)> = None;
        let mut current = isize::from(std::i16::MIN);
        let mut last_move = (0, 0);
        loop {
            state.next_move(last_move.0, last_move.1);
            let new_move = match state.selected_move {
                Some(new_move) => new_move,
                None => break,
            };
            let mut new_state = state.clone();
            new_state.make_move(new_move.0, new_move.1, stone);
            let mut score = -self.negascout(&mut new_state, stone.opposant(), depth - 1, -(alpha + 1), -alpha);
            if score > alpha && score < beta {
                score = -self.negascout(&mut new_state, stone.opposant(), depth - 1, -beta, -alpha);
            }
            if score > current {
                current = score;
                best_move = Some(new_move);
                alpha = score.max(alpha);
                if alpha >= beta {
                    break;
                }
            }
            last_move = (new_move.0 + 1, new_move.1);
        }
        state.selected_move = best_move;
        alpha
    }
    
    // pub fn negascout(&self, state: &mut Gameboard, stone: Stone, depth: u8, mut alpha: isize, beta: isize) -> isize {
    //     if depth == 0 || self.is_victory() {
    //         return self.eval(state);
    //     }
    //     let mut best_move: Option<(usize, usize)> = None;
    //     let mut current = isize::from(std::i16::MIN);
    //     for new_move in state.expand() {
    //         let mut new_state = state.clone();
    //         new_state.make_move(new_move.0, new_move.1, stone);
    //         let mut score = -self.negascout(&mut new_state, stone.opposant(), depth - 1, -(alpha + 1), -alpha);
    //         if score > alpha && score < beta {
    //             score = -self.negascout(&mut new_state, stone.opposant(), depth - 1, -beta, -alpha);
    //         }
    //         if score > current {
    //             current = score;
    //             best_move = Some(new_move);
    //             alpha = score.max(alpha);
    //             if alpha >= beta {
    //                 break;
    //             }
    //         }
    //     }
    //     state.selected_move = best_move;
    //     alpha
    // }

    // pub fn negascout(&self, state: &mut Gameboard, stone: Stone, depth: u8, mut alpha: isize, beta: isize) -> isize {
	// 	// println!("negascout");
	// 	// let mut all_eval: Vec<((usize, usize), isize)> = Vec::new();
    //     if depth == 0 || self.is_victory() {
    //         return self.eval(state);
    //     }
    //     let original_possible_moves = state.possible_moves;
    //     state.next_move(0,0);
    //     if state.selected_move.is_none() {
    //         return self.eval(state);
    //     }
    //     let mut best_move: (usize, usize) = state.selected_move.unwrap();
    //     let mut last_move = best_move;

    //     state.make_move(best_move.0, best_move.1, stone);
    //     let mut current = -self.negascout(state, stone.opposant(), depth - 1, -beta, -alpha);
	// 	// all_eval.push((best_move, current));
    //     state.unmake_move(best_move.0, best_move.1);
    //     state.possible_moves = original_possible_moves;
    //     if current >= alpha {
    //         alpha = current;
    //     }
    //     if current < beta {
    //         loop {
    //             state.next_move(last_move.0 + 1, last_move.1);
    //             if state.selected_move.is_none() {
    //                 break;
    //             }
    //             last_move = state.selected_move.unwrap();
    //             state.make_move(last_move.0, last_move.1, stone);
    //             let mut score = -self.negascout(state, stone.opposant(), depth - 1, -(alpha + 1), -alpha);
	// 			// all_eval.push((last_move, score));
    //             if score > alpha && score < beta {
    //                 score = -self.negascout(state, stone.opposant(), depth - 1, -beta, -alpha);
	// 				// all_eval.push((last_move, score));
    //             }
    //             state.unmake_move(last_move.0, last_move.1);
    //             state.possible_moves = original_possible_moves;
    //             if score > current {
    //                 current = score;
    //                 best_move = last_move;
    //                 if score > alpha {
    //                     if score >= beta {
    //                         break;
    //                     }
    //                     alpha = score;
    //                 }
    //             }
    //         }
    //     }
    //     state.selected_move = Some(best_move);
	// 	// print_all_state(all_eval, state);
    //     current
    // }
}

pub fn print_all_state(all_eval: Vec<((usize, usize), isize)>, state: &Gameboard) {
	let mut print: bool;
	print!("ALL STATES: \n   ");
	for x in 0..SIZE {
		print!("{0: <2} ", x);
	}
	println!();
	for y in 0..SIZE {
			print!("{0: <2} ", y);
			for x in 0..SIZE {
				print = false;
				'geteval: for elem in &all_eval {
					if elem.0 == (x as usize, y as usize) {
						print!("{0: <3}", elem.1);
						print = true;
						break 'geteval;
					}
				}
				if !print {
					if state.cells[x][y] == Stone::WHITE {
						print!(" {}[7;49;97mW{}[0m ", 27 as char, 27 as char);
					} else if state.cells[x][y] == Stone::BLACK {
						print!(" {}[7;49;90mB{}[0m ", 27 as char, 27 as char);
					} else {
						print!(".  ");
					}
				}
			}
			println!();
		}
}