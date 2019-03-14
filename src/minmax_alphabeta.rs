use crate::models::gameboard::*;

pub fn algo(gameboard: &mut Gameboard, current_stone: Stone) -> (isize, Option<Gameboard>) {
	let alpha = std::isize::MIN;
	let beta = std::isize::MAX;

	let next_stone: Stone;
	if current_stone == Stone::BLACK {
		next_stone = Stone::WHITE;
	} else {
		next_stone = Stone::BLACK;
	}
	alphabeta(gameboard, 3, alpha + 1, beta, true, current_stone, next_stone)
}

pub fn count_value_of_align(align: &Alignement, gameboard: &Gameboard) -> usize {
	let mut value = 0;

	let len = len_of_one_align(align);
	match align.before {
		Some(before) => {
			if gameboard.cells[before[0]][before[1]] == Stone::NOPE {
				value += 15;
			}
		},
		_ => (),
	}
	match align.after {
		Some(after) => {
			if gameboard.cells[after[0]][after[1]] == Stone::NOPE {
				value += 15;
			}
		},
		_ => (),
	}
	value += (5 * len);
		if len > 4 {
			value += 1000;
	} else if (len > 2)
	{
		value += 30;
	}
	// if len > 4 {
	// 	value *= 2;

	value
}


pub fn eval(gameboard: & Gameboard, stone: Stone) -> isize {
	// let range_h:Vec<usize> = (gameboard.win[0]..gameboard.win[2] as usize).collect();
	// let range_v:Vec<usize> = (gameboard.win[1]..gameboard.win[3] as usize).collect();

	// println!("---------------------------------------\n\t\tALIGN WHITE: gameboard.align_list_white: len: {}", gameboard.align_list_white.len());
	// dbg!(&gameboard.align_list_white);
	// println!("---------------------------------------\n\t\tALIGN BLACK: gameboard.align_list_white: len: {}", gameboard.align_list_white.len());
	// dbg!(&gameboard.align_list_black);

	println!("EVAL:");
	gameboard.printboard();


	let mut value: isize = 0;
	if stone == Stone::WHITE {
		for align in &gameboard.align_list_white {
			println!("\talign WHITE(positif): ");
			align.print_align();
			let test = count_value_of_align(align, gameboard) as isize;
			value += test;
			println!("\tvalue: {}", test);
		}
		for align in &gameboard.align_list_black {
			println!("\talign BLACK(negatif): ");
			align.print_align();
			let test = count_value_of_align(align, gameboard) as isize;
			value -= test;
			println!("\tvalue: (-){}", test);
		}
	}
	else {
		for align in &gameboard.align_list_white {
			println!("\talign WHITE(negatif): ");
			align.print_align();
			let test = count_value_of_align(align, gameboard) as isize;
			value -= test;
			println!("\tvalue: (-){}", test);
		}
		for align in &gameboard.align_list_black {
			println!("\talign BLACK(positif): ");
			align.print_align();
			let test = count_value_of_align(align, gameboard) as isize;
			value += test;
			println!("\tvalue: {}", test);
		}
	}
	println!("finalValue: {}", value);
	value
}

pub fn alphabeta(gameboard: & Gameboard, depth: i32, mut alpha: isize, mut beta: isize, noeud_max: bool, stone: Stone, next_stone: Stone) -> (isize, Option<Gameboard>) {
	if depth <= 0 || gameboard.victory() == true {
		return (eval(gameboard, stone), None);
	}
	let mut best_board = gameboard.clone();

	// // // NEGA MAX ALPHA BETA? not work ?
	// for new_board in gameboard.expand() {
	// 	let (mut score, _) = alphabeta(&new_board, depth - 1, -beta, -alpha, !noeud_max, stone, next_stone);
	// 	score = -score;
	// 	if score >= alpha {
	// 			println!("_________ new better board: for NEGAMAX (score: {})(stone: {:?}) ", score, stone);
	// 			printboard(&new_board);
	// 		alpha = score;
	// 		best_board = new_board.clone();
	// 		if alpha >= beta {
	// 			break ;
	// 		}
	// 	}
	// }
	// (alpha, Some(best_board))

	// MIN MAX ALPHA-BETA
	if noeud_max == true {
		for new_board in gameboard.expand(stone) {

			let (score, _) = alphabeta(&new_board, depth - 1, alpha, beta, !noeud_max, stone, next_stone);
			if score > alpha {
				alpha = score;
				best_board = new_board.clone();
				if alpha >= beta {
					break ;
				}
			}
		}
	(alpha, Some(best_board))
	} else {
		for new_board in gameboard.expand(next_stone) {
			let (score, _) = alphabeta(&new_board, depth - 1, alpha, beta, !noeud_max, stone, next_stone);
			if score < beta {
				beta = score;
				best_board = new_board.clone();
				if alpha >= beta {
					break ;
				}
			}
		}
	(beta, Some(best_board))
	}
}
