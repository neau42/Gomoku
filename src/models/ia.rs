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
        let original_possible_moves = state.possible_moves;
        state.next_move(0,0);
        if state.selected_move.is_none() {
            return self.eval(state);
        }
        let mut best_move: (usize, usize) = state.selected_move.unwrap();
        let mut last_move = best_move;

        state.make_move(best_move.0, best_move.1, stone);
        let mut current = -self.negascout(state, stone.opposant(), depth - 1, -beta, -alpha);
        state.unmake_move(best_move.0, best_move.1);
        state.possible_moves = original_possible_moves;
        if current >= alpha {
            alpha = current;
        }
        if current < beta {
            loop {
                state.next_move(last_move.0 + 1, last_move.1);
                if state.selected_move.is_none() {
                    break;
                }
                last_move = state.selected_move.unwrap();
                state.make_move(last_move.0, last_move.1, stone);
                let mut score = -self.negascout(state, stone.opposant(), depth - 1, -(alpha + 1), -alpha);
                if score > alpha && score < beta {
                    score = -self.negascout(state, stone.opposant(), depth - 1, -beta, -alpha);
                }
                state.unmake_move(last_move.0, last_move.1);
                state.possible_moves = original_possible_moves;
                if score > current {
                    current = score;
                    best_move = last_move;
                    if score > alpha {
                        if score >= beta {
                            break;
                        }
                        alpha = score;
                    }
                }
            }
        }
        state.selected_move = Some(best_move);
        current
    }
}