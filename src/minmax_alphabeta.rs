use crate::models::gameboard::*;

pub fn algo(gameboard: &mut Gameboard, current_stone: Stone) -> (isize, Option<Gameboard>) {
	let alpha = std::isize::MIN + 1;
	let beta = std::isize::MAX;

	let next_stone: Stone;
	if current_stone == Stone::BLACK {
		next_stone = Stone::WHITE;
	} else {
		next_stone = Stone::BLACK;
	}
	alphabeta(gameboard, 4, alpha, beta, true, current_stone, next_stone)
}

pub fn eval(gameboard: & Gameboard, is_max: bool) -> isize {
	if !is_max {
		gameboard.value as isize
	}
	else {
		- (gameboard.value as isize)

	}
}

pub fn alphabeta(gameboard: & Gameboard, depth: i32, mut alpha: isize, mut beta: isize, is_max: bool, stone: Stone, next_stone: Stone) -> (isize, Option<Gameboard>) {
	/*if depth <= 0 || gameboard.victory() == true {
		return (eval(gameboard, is_max), None);
	}
	let mut best_board = gameboard.clone();

	// // // NEGA MAX ALPHA BETA - work ?

	// let mut best_value = std::isize::MIN + 1;
	// for new_board in gameboard.expand(stone) {
	// 	let (mut score, _) = alphabeta(&new_board, depth - 1, -beta, -alpha, !is_max, next_stone, stone);
	// 	score = -score;
	// 	if score > best_value {
	// 		best_value = score;
	// 		best_board = new_board.clone();
	// 		if best_value > alpha {
	// 			alpha = best_value;
	// 			if alpha > beta {break;}
	// 		}
	// 	}
	// }
	// (best_value, Some(best_board)

	// MIN MAX ALPHA-BETA
	if is_max {
		for new_board in gameboard.expand(stone) {

			let (score, _) = alphabeta(&new_board, depth - 1, alpha, beta, !is_max, stone, next_stone);
			if score > alpha {
				alpha = score;
				best_board = new_board.clone();
				if alpha >= beta {
					break ;
				}
			}
		}
	// println!("+++++ DEPTH: {} ++++ MAX FIND FOR: ", depth);
	// best_board.printboard();
	// println!("+++++ max value: {}, max: {}", best_board.value, best_board.max);
	(alpha, Some(best_board))
	} else {
		for new_board in gameboard.expand(next_stone) {
			let (score, _) = alphabeta(&new_board, depth - 1, alpha, beta, !is_max, stone, next_stone);
			if score < beta {
				beta = score;
				best_board = new_board.clone();
				if alpha >= beta {
					break ;
				}
			}
		}
	// println!("----- DEPTH: {} ---- MIN FIND FOR: ", depth);
	// best_board.printboard();
	// println!("----- min value: {}, max: {}", best_board.value, best_board.max);
	(beta, Some(best_board))
	}*/
	(0, None)
}
