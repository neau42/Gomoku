use crate::models::gameboard::*;
use std::collections::HashMap;
use crate::eval::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IA {
    pub depth: u8,
}


	pub fn filter_boards(possible_boards: Vec<Gameboard>, stone: u8) -> Vec<Gameboard> {
		let closure = |board: &Gameboard| {
				if (stone == BLACK && 
					(board.priority == Priority::BlackWin
					|| board.priority == Priority::BlackWin1
					|| board.priority == Priority::BlackWin2))
					|| (stone == WHITE && (
					board.priority == Priority::WhiteWin
					|| board.priority == Priority::WhiteWin1
					|| board.priority == Priority::WhiteWin2)) {
						true
					}
					else {
						false
					}
			};
		if possible_boards.iter().filter(|board| closure(board)).count() > 0 {
			possible_boards.into_iter().filter_map(|board| { 
					if closure(&board) {
						Some(board)
					}
					else {
						None
					}
				}).collect()
		}
		else {
			possible_boards
		}
	}

impl IA {
    pub fn new(depth: u8) -> IA {
        IA {
            depth,
        }
    }

	pub fn expand(self, state: &Gameboard, stone: u8, depth: u8, map_board_values: &mut HashMap<[u64; SIZE], isize>, player_stone: u8) -> Vec<Gameboard> {
		let possible_moves: Vec<(usize, usize)> = state.expand();
        let possible_boards: Vec<Gameboard> = possible_moves.iter().filter_map(|new_move| {
			let mut new_state = state.clone();
            if new_state.make_move(new_move.0, new_move.1, stone) {
                eval(&mut new_state, opposite_stone!(stone), depth - 1, map_board_values, player_stone);
				Some(new_state)
			}
			else {
				None
			}
		}).collect();
		let mut possible_boards = filter_boards(possible_boards, stone);
		possible_boards.sort_by(|board, other| board.value.cmp(&other.value));

		let t = [80, 60, 50, 45, 40, 35, 30, 25, 20, 20];

		if self.depth >= 5 {
			possible_boards[0..(possible_boards.len() * t[(self.depth - depth) as usize] / 100)].to_vec()
		} else {
			possible_boards
		}
	}
	/// si alpha < current < beta, alors current est la valeur minimax
    /// si current <= alpha, alors la vraie valeur minimax m vérifie : m <= current <= alpha
    /// si beta <= current alors la vraie valeur minimax m vérifie : beta <= current <= m
pub fn negascout(&mut self, state: &mut Gameboard, stone: u8, depth: u8, mut alpha: isize, beta: isize, map_board_values: &mut HashMap<[u64; SIZE], isize>, all_values: &mut Vec<(usize, usize, isize)>, player_stone: u8) -> isize {
		if depth == 0 || state.is_finish() {
			return state.value;
        }
        let mut best_move: Option<(usize, usize)> = None;
        let mut current = (std::i64::MIN + 1) as isize;
		let mut tmp_beta = beta;
		let possible_states: Vec<Gameboard> = self.expand(state, stone, depth, map_board_values, player_stone);
        for (i, mut new_state) in possible_states.into_iter().enumerate() {
            let mut score = -self.negascout(&mut new_state, opposite_stone!(stone), depth - 1, -tmp_beta, -alpha, map_board_values, all_values, player_stone);
            if score > alpha && score < beta && i > 0 && depth > 1 {
                score = -self.negascout(&mut new_state, opposite_stone!(stone), depth - 1, -beta, -alpha, map_board_values, all_values, player_stone);
            }
			if depth == self.depth {
				all_values.push((new_state.last_move.unwrap().0, new_state.last_move.unwrap().1, score));
			}
            if score > current {
                current = score;
                best_move = new_state.last_move;
                alpha = score.max(alpha);
                if alpha >= beta {
                    break;
                }
				tmp_beta = alpha + 1;
            }
        }
        state.selected_move = best_move;
        current
    }
}