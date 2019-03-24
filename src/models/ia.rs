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
    pub fn is_victory(&self) -> bool {
        false
    }

    pub fn eval(&self, state: &Gameboard, stone: u8) -> isize {
		println!("\n\n______ EVAL _______");
		printboard!(&state.cells);



		// let horizontal: u32 = line_horizontal!(self.cells, x_min, x_max, y as usize);
		// let vertical: u32 = line_vertical!(self.cells[x as usize], y_min, y_max);
		// let down_diago: u32 = down_diago!(self.cells, x as usize, x_min, x_max, y as usize, y_min, y_max);
		// let up_diago: u32 = up_diago!(self.cells, x as usize, x_min, x_max, y as usize, y_min, y_max);
		
		// let list: [u32; 4] = [horizontal, vertical, down_diago, up_diago];





		let test: isize = eval!(state.cells, stone);
		// let test =  eval_line!(state.cells[0]);
		println!("TEST: {:?}", test);
        0
    }

}

impl IA {
	/// si alpha < current < beta, alors current est la valeur minimax
    /// si current <= alpha, alors la vraie valeur minimax m vérifie : m <= current <= alpha
    /// si beta <= current alors la vraie valeur minimax m vérifie : beta <= current <= m
    pub fn negascout(&self, state: &mut Gameboard, stone: u8, depth: u8, mut alpha: isize, beta: isize) -> isize {
        if depth == 0 || self.is_victory() {
            return self.eval(state, stone);
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
            let mut score = -self.negascout(&mut new_state, opposite_stone!(stone), depth - 1, -(alpha + 1), -alpha);
            if score > alpha && score < beta {
                score = -self.negascout(&mut new_state, opposite_stone!(stone), depth - 1, -beta, -alpha);
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

    pub fn alphabeta(&self, state: &mut Gameboard, stone: u8, depth: u8, mut alpha: isize, beta: isize) -> isize {
        if depth == 0 || self.is_victory() {
            return self.eval(state, stone);
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
            let mut score = -self.alphabeta(&mut new_state, opposite_stone!(stone), depth - 1, -beta, -alpha);
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
}