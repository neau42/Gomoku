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

pub fn evale_one_line(mut line: u64, stone: u8) -> isize {
	let mut value = 0;
	let mut i: isize = 0;
	let mut j: isize = 0;

	while i < 64 {
		match (line & 0b1111_1111_1111) as u16 {
			0b0000_0000_0000 => {  // ALIGN NULL
					j = 10;
					i += 10;
			},
			0b0110_1010_1000 | 0b0010_1010_1001 => {  // ALIGN 4/
					value -= 10000;
					j = 10;
					i += 10;
			},
			// 0b1010_1010_1000
			0b0010_1010_1010 | 0b1010_1010_1000 | 0b1010_1010_1001 | 0b0110_1010_1010 => {  // ALIGN 5
					value -= 10000000;
					j = 10;
					i += 10;
			},
			// is_open if (is_open & 0b0011_1111_1100) != 0 => {

			// }
			0b0010_1010_1000 => {  // ALIGN 4
					value -= 100000;
					j = 10;
					i += 10;
			},
			0b0010_0010_1000 | 0b0010_1000_1000 | 0b0000_1010_1000 => { // ALIGN 3
					value -= 1000;
					j = 10;
					i += 10;
			},
			0b0010_0010_1001 | 0b0010_1000_1001 | 0b0000_1010_1001 => { // ALIGN 3/
					value -= 100;
					j = 10;
					i += 10;
			},
			0b0110_0010_1000 | 0b0110_1000_1000 | 0b0100_1010_1000 => { // ALIGN /3
					value -= 100;
					j = 10;
					i += 10;
			},
			0b0000_1010_0000 => { //ALIGN 2
					value -= 100;
					j = 10;
					i += 10;
			},

			0b0001_0101_0101 | 0b0101_0101_0100 | 0b0101_0101_0110 | 0b1001_0101_0101 => {  // ALIGN 5
					value += 10000000;
					j = 10;
					i += 10;
			},
			0b1001_0101_0100 | 0b0001_0101_0110 => {  // ALIGN 4/
					value += 10000;
					j = 10;
					i += 10;
			},
			0b0001_0101_0100 => {  // ALIGN 4
					value += 100000;
					j = 10;
					i += 10;
			},
			0b0001_0100_0100 | 0b0001_0001_0100 | 0b0000_0101_0100 => { // ALIGN 3
					value += 1000;
					j = 10;
					i += 10;
			},
			0b0001_0100_0110 | 0b0001_0001_0110 | 0b0000_0101_0110 => { // ALIGN 3/
					value += 100;
					j = 10;
					i += 10;
			},
			0b1001_0100_0100 | 0b1001_0001_0100 | 0b1000_0101_0100 => { // ALIGN /3
					value += 100;
					j = 10;
					i += 10;
			},

			0b0000_0101_0000 => { //ALIGN 2
					value += 100;
					j = 10;
					i += 10;
			},
			_ => {
					j = 2;
					i += 2;
			},
		}
		line >>= j;
	}
	if stone == WHITE {
		// println!("±value: {} ",-value);
		-value
	} else {
		// println!("±value: {} ",value);
		value
	}
}

impl IA {
    pub fn is_victory(&self) -> bool {
        false
    }

    pub fn eval(&self, state: &Gameboard, stone: u8) -> isize {
		// println!("\n\n______ EVAL _______");
		// printboard!(&state.cells);


		let mut all: Vec<u64> = (0..SIZE).map(|y| line_horizontal!(state.cells, 0, SIZE - 1, y as usize)).collect();
		let all_verti: Vec<u64> = (0..SIZE).map(|x| line_vertical!(state.cells[x as usize], 0 , SIZE -1)).collect();
		let all_diag_d: Vec<u64> = (0..SIZE).map(|x| down_diago_orig!(state.cells, x as usize, 0, SIZE - 1, 0, 0, SIZE - 1)).collect();
		// let all_diag_d_test: Vec<u64> = (0..SIZE).map(|x| down_diago!(state.cells, 0, SIZE - 1, x, 0)).collect();
		let all_diag_d2: Vec<u64> = (1..SIZE).map(|y| down_diago_orig!(state.cells, SIZE - 1, 0, SIZE - 1, y as usize, 0, SIZE - 1)).collect();
		// let all_diag_d2_test: Vec<u64> = (1..SIZE).map(|y| down_diago!(state.cells, 0, SIZE - 1, 0, y)).collect();
		let all_diag_u: Vec<u64> = (0..SIZE).map(|x| up_diago_orig!(state.cells, x as usize, 0, SIZE -1, 0, 0, SIZE - 1)).collect();
		let all_diag_u2: Vec<u64> = (1..SIZE).map(|y| up_diago_orig!(state.cells, 0, 0, SIZE -1, y as usize, 0, SIZE - 1)).collect();




		// dbg!(&all_diag_d);
		// dbg!(&all_diag_d_test);
		// dbg!(&all_diag_d2);
		// dbg!(&all_diag_d2_test);

		all.extend(all_verti);
		all.extend(all_diag_d);
		all.extend(all_diag_d2);
		all.extend(all_diag_u);
		all.extend(all_diag_u2);
		all.retain(|&elem| elem != 0);

		let mut value: isize = 0;

		for e in all {
			// println!("{:#066b}", e);
			value += evale_one_line(e, stone);
		}

		// let test: isize = eval!(state.cells, stone);
		// let test =  eval_line!(state.cells[0]);
		// println!("value: {:?}", value);
        value
    }

}

impl IA {
	/// s6 alpha < current < beta, alors current est la valeur minimax
    /// si current <= alpha, alors la vraie valeur minimax m vérifie : m <= current <= alpha
	/// 	/// s6 alpha < current < beta, alor:::: est la valeur minimax
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