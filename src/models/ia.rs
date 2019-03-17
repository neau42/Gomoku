use crate::models::gameboard::*;

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

    pub fn eval(&self, gameboard: & Gameboard) -> isize {
		// 0
		// println!("\n\n_____________");
		// gameboard.printboard();
		// println!("eval: {}", gameboard.value);

		-gameboard.value
    }

    /// si alpha < current < beta, alors current est la valeur minimax
    /// si current <= alpha, alors la vraie valeur minimax m vérifie : m <= current <= alpha
    /// si beta <= current alors la vraie valeur minimax m vérifie : beta <= current <= m
    // pub fn negascout(&self, state: &mut Gameboard, stone: &Stone, depth: u8, mut alpha: isize, beta: isize) -> (isize, Option<(usize, usize)>) {
    //     if self.is_victory() ||  depth <= 0 {
    //         return (self.eval(), None);
    //     }
    //     let mut all_move: Vec<(usize, usize)> = state.expand(*stone);
    //     if all_move.is_empty() {
    //         return (self.eval(), None);
    //     }
    //     let mut best_move: (usize, usize) = all_move.pop().unwrap();
    //     state.make_move(best_move.0, best_move.1, *stone);
    //     let mut current = -self.negascout(state, stone, depth - 1, -beta, -alpha).0;
    //     state.unmake_move(best_move.0, best_move.1);
    //     if current >= alpha {
    //         alpha = current;
    //     }
    //     if current < beta {
    //         for single_move in all_move {
    //             state.make_move(single_move.0, single_move.1, *stone);
    //             let mut score = -self.negascout(state, stone, depth - 1, -(alpha + 1), -alpha).0;
    //             if score > alpha && score < beta {
    //                 score = -self.negascout(state, stone, depth - 1, -beta, -alpha).0;
    //             }
    //             state.unmake_move(single_move.0, single_move.1);
    //             if score >= current {
    //                 current = score;
    //                 best_move = single_move;
    //                 if score >= alpha {
    //                     alpha = score;
    //                     if score >= beta {
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return (current, Some(best_move));
    // }

    pub fn negascout(&self, state: &mut Gameboard, stone: &Stone, depth: u8, mut alpha: isize, beta: isize) -> (isize, Option<(usize, usize)>) {
        if self.is_victory() ||  depth <= 0 {
            return (self.eval(state), None);
        }
		// state.printboard();
        let best_move = state.next_move(None);
        if best_move.is_none() {
            return (self.eval(state), None);
        }
        let mut last_move = best_move.clone();
        let mut best_move = best_move.unwrap();
        
        state.make_move(best_move.0, best_move.1, *stone);
        let mut current = -self.negascout(state, stone, depth - 1, -beta, -alpha).0;
        state.unmake_move(best_move.0, best_move.1);
        if current >= alpha {
            alpha = current;
        }
        if current < beta {
            'move_loop: loop {
                let single_move = state.next_move(last_move);
                if single_move.is_none() {
                    break 'move_loop;
                }
                last_move = single_move.clone();
                let single_move = single_move.unwrap();
                state.make_move(single_move.0, single_move.1, *stone);
                let mut score = -self.negascout(state, stone, depth - 1, -(alpha + 1), -alpha).0;
                if score > alpha && score < beta {
                    score = -self.negascout(state, stone, depth - 1, -beta, -alpha).0;
                }
                state.unmake_move(single_move.0, single_move.1);
                if score >= current {
                    current = score;
                    best_move = single_move;
                    if score >= alpha {
                        alpha = score;
                        if score >= beta {
                            break;
                        }
                    }
                }
            }
        }
        return (current, Some(best_move));
    }
}
