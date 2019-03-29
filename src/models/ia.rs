use crate::models::gameboard::*;
use std::collections::HashSet;
use std::collections::HashMap;
// use std::cmp::min;
// use std::cmp::max;
// use std::process::exit;
use crate::eval::*;

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

    // pub fn is_victory(&self) -> bool {
    //     false
    // }

	/// si alpha < current < beta, alors current est la valeur minimax
    /// si current <= alpha, alors la vraie valeur minimax m vérifie : m <= current <= alpha
    /// si beta <= current alors la vraie valeur minimax m vérifie : beta <= current <= m
    pub fn negascout(&self, state: &mut Gameboard, transposition_table: &mut HashSet<Gameboard>, stone: u8, depth: u8, mut alpha: isize, beta: isize, map_board_values: &mut HashMap<[u64; SIZE], isize>, all_values: &mut HashMap<(usize, usize), isize>) -> isize {
        // if depth % 2 == 0 && transposition_table.contains(state) {
		// 	state.value = transposition_table.get(state).unwrap().value;			
		// 	return state.value
		// }

		if depth == 0 || state.is_finish() {
			state.value = eval(state, stone, depth, map_board_values);
			// if depth % 2 == 0 {
			// 	transposition_table.insert(state.clone());
			// }
            return state.value;
        }
        let mut best_move: Option<(usize, usize)> = None;
        let mut current = (std::i64::MIN + 1) as isize;
        let mut last_move = (0, 0);
		let mut tmp_beta = beta;
		let mut i = 0;
        loop {
            state.next_move(last_move.0, last_move.1);
            let new_move = match state.selected_move {
                Some(new_move) => new_move,
                None => break,
            };
            let mut new_state = state.clone();
            new_state.make_move(new_move.0, new_move.1, stone);
            let mut score = -self.negascout(&mut new_state, transposition_table, opposite_stone!(stone), depth - 1, -tmp_beta, -alpha, map_board_values, all_values);
            if score > alpha && score < beta && i > 0 && depth > 1 {
                score = -self.negascout(&mut new_state, transposition_table, opposite_stone!(stone), depth - 1, -beta, -alpha, map_board_values, all_values);
            }
			all_values.insert((new_move.0, new_move.1), score);
			i += 1;
            if score > current {
                current = score;
                best_move = Some(new_move);
                alpha = score.max(alpha);
                if alpha >= beta {
                    break;
                }
				tmp_beta = alpha + 1;
            }
            last_move = (new_move.0 + 1, new_move.1);
        }
        state.selected_move = best_move;
        current
    }

    // pub fn alphabeta(&self, state: &mut Gameboard, transposition_table: &mut HashSet<Gameboard>, stone: u8, depth: u8, mut alpha: isize, beta: isize, map_board_values: &mut HashMap<[u64; SIZE], isize>) -> isize {
    //     if depth % 2 == 0 && transposition_table.contains(state) {
	// 		state.value = transposition_table.get(state).unwrap().value;
	// 		return state.value
	// 	}
	// 	if depth == 0 || state.is_finish() {
	// 		state.value = eval(state, stone, depth, map_board_values);
	// 		if depth % 2 == 0 {
	// 			transposition_table.insert(state.clone());
	// 		}
    //         return state.value;
    //     }
    //     let mut best_move: Option<(usize, usize)> = None;
    //     let mut current = isize::from(std::i16::MIN);
    //     let mut last_move = (0, 0);
    //     loop {
    //         state.next_move(last_move.0, last_move.1);
    //         let new_move = match state.selected_move {
    //             Some(new_move) => new_move,
    //             None => break,
    //         };
    //         let mut new_state = state.clone();
    //         new_state.make_move(new_move.0, new_move.1, stone);
    //         let score = -self.alphabeta(&mut new_state, transposition_table, opposite_stone!(stone), depth - 1, -beta, -alpha, map_board_values);
    //         if score > current {
    //             current = score;
    //             best_move = Some(new_move);
    //             alpha = score.max(alpha);
    //             if alpha >= beta {
    //                 break;
    //             }
    //         }
    //         last_move = (new_move.0 + 1, new_move.1);
    //     }
    //     state.selected_move = best_move;
    //     alpha
    // }
}