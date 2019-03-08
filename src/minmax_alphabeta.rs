use crate::models::gameboard::{Gameboard, Stone, SIZE};

use rand::Rng;
// const SIZE: usize = 19;

pub fn algo(gameboard: &mut Gameboard) {
	let alpha = std::isize::MAX;
	let beta = std::isize::MIN;

	for y in 0..SIZE {
		for x in 0..SIZE {
			match gameboard.cells[x][y] {
				Stone::WHITE => gameboard.white_stone[x][y] = true,
				Stone::BLACK => gameboard.black_stone[x][y] = true,
				_ => (),
			}
		}
	}
	printboard(&gameboard);
	alphabeta(gameboard,1 ,std::isize::MIN + 1, std::isize::MAX, true);
}


pub fn printboard(gameboard: & Gameboard) {
	println!("BOARD: ");
	for y in 0..SIZE {
		for x in 0..SIZE {
			match gameboard.cells[x][y] {
				Stone::WHITE => print!("x "),
				Stone::BLACK => print!("o "),
				_ => print!(". ")
			}
		}
		println!("");
	}
}



pub fn eval_border(cells: [[Stone; SIZE]; SIZE], x: usize, y: usize, black_turn: bool) -> isize {
	let mut cmpt = 0;
	let stone_ref;
	let other_stone;

	if black_turn == true {
		stone_ref = Stone::BLACK;
		other_stone = Stone::WHITE;
	} else {
		stone_ref = Stone::WHITE;
		other_stone = Stone::BLACK;

	};
	if cells[x][y] == stone_ref {
		if x > 0 && y > 0 && x < SIZE - 1 && y < SIZE - 1 {
			if cells[x - 1][y - 1] == stone_ref { cmpt += 2;}
			if cells[x][y - 1] == stone_ref { cmpt += 2;}
			if cells[x - 1][y] == stone_ref { cmpt += 2;}
			if cells[x + 1][y + 1] == stone_ref { cmpt += 2;}
			if cells[x][y + 1] == stone_ref { cmpt += 2;}
			if cells[x + 1][y] == stone_ref { cmpt += 2;}
			
			if cells[x - 1][y - 1] == other_stone { cmpt += 2;}
			if cells[x][y - 1] == other_stone { cmpt += 2;}
			if cells[x - 1][y] == other_stone { cmpt += 2;}
			if cells[x + 1][y + 1] == other_stone { cmpt += 2;}
			if cells[x][y + 1] == other_stone { cmpt += 2;}
			if cells[x + 1][y] == other_stone { cmpt += 2;}
		}
	}
	// if cmpt > 0 {
	// 	println!("+1 for x:{} y:{}", x, y);
	// }
		cmpt
}


pub fn eval(gameboard: & Gameboard, black_turn: bool) -> isize {
	let mut cmpt = 0;

	// println!("before eval, blackturn? {}", black_turn);

	for y in 0..SIZE {
		for x in 0..SIZE {
			cmpt += eval_border(gameboard.cells, x, y, !black_turn);
		}
	}
	// println!("EVAL: {}", cmpt);
	cmpt
}

pub fn alphabeta(gameboard: & Gameboard, depth: i32, mut alpha: isize, mut beta: isize, black_turn: bool) -> (isize, Option<Gameboard>) {
	// println!("alphabeta: ALPHA:{}, BETA:{}", alpha, beta);

	let mut new_board = gameboard.clone();

	// if (game over or depth <= 0)
	if depth <= 0 {
	return (eval(gameboard, black_turn), None);
	}
	let mut best_board;

	if black_turn == true {
		for y in 0..SIZE {
			for x in 0..SIZE {
				if gameboard.cells[x][y] == Stone::NOPE {
					new_board = gameboard.clone();
					new_board.cells[x][y] = Stone::BLACK;
					let (score, _) = alphabeta(&new_board, depth - 1, alpha, beta, false);
					if score > alpha {
						alpha = score;
						best_board = new_board.clone();
							println!("0000 best board");
							printboard(&best_board);
						if alpha >= beta {
							break ;
						}
					}
				}
			}
		}
	}
	else {
		for y in 0..SIZE {
			for x in 0..SIZE {
				if gameboard.cells[x][y] == Stone::NOPE {
					new_board = gameboard.clone();
					new_board.cells[x][y] = Stone::WHITE;
					let (score, _) = alphabeta(&new_board, depth - 1, alpha, beta, true);
					if score < beta {
						beta = score;
						best_board = new_board.clone();
							println!("1111 best board:");
							printboard(&best_board);

						if alpha >= beta {
							break ;
						}
					}
				}
			}
		}

	}
	(alpha, Some(new_board))
}
//    for (each possible move m) {
//       make move m;
//       int score = -alphabêta(depth -
// 	   1, -bêta, -alpha)
//       unmake move m;
//       if (score >= alpha){
//          alpha = score ;
//          bestMove = m ;
//          if (alpha >= bêta)
//             break;
//       }
//    }
//    return alpha;
// 	0
// }