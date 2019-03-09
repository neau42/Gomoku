use crate::models::gameboard::{Gameboard, Stone, SIZE};

use rand::Rng;
// const SIZE: usize = 19;

pub fn algo(gameboard: &mut Gameboard, current_stone: Stone) -> (isize, Option<Gameboard>) {
	let alpha = std::isize::MIN;
	let beta = std::isize::MAX;

	// let mut white_stone: [[bool; SIZE]; SIZE] = [[false; SIZE]; SIZE];
	// let mut black_stone: [[bool; SIZE]; SIZE] = [[false; SIZE]; SIZE];
	// for y in 0..SIZE {
	// 	for x in 0..SIZE {
	// 		match gameboard.cells[x][y] {
	// 			Stone::WHITE => white_stone[x][y] = true,
	// 			Stone::BLACK => black_stone[x][y] = true,
	// 			_ => (),
	// 		}
	// 	}
	// }
	let next_stone: Stone;
	if current_stone == Stone::BLACK {
		next_stone = Stone::WHITE;
	} else {
		next_stone = Stone::BLACK;
	}
	printboard(&gameboard);
	alphabeta(gameboard, 1, alpha + 1, beta, true, current_stone, next_stone)
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



pub fn eval_around(cells: [[Stone; SIZE]; SIZE], x: isize, y: isize, stone: Stone) -> Option<isize> {
	if cells[x as usize][y as usize] == Stone::NOPE { return None;}
	// println!("eval_around: x{}, y{} => {:?}", x, y, cells[x as usize][y as usize]);
	let mut cmpt = 0;
	let other_stone;
	let around: [(isize, isize); 8] = [(x,y+1),
		(x+1, y+1),
		(x+1, y),
		(x+1, y -1),
		(x, y -1),
		(x-1, y-1),
		(x-1, y),
		(x-1, y+1)];

	if stone == Stone::BLACK {
		 other_stone = Stone::WHITE;
	} else {
		 other_stone = Stone::BLACK;
	}
	for (x, y) in around.iter().filter(|(x, y)| *x >= 0 && *y >= 0 && *x < 19 && *y < 19) {
				if cells[*x as usize][*y as usize] == stone {
					cmpt = (3 + cmpt);
				} else if cells[*x as usize][*y as usize] == other_stone {
					cmpt = (1 + cmpt);
				}
	}
	Some(cmpt)
}

pub fn eval(gameboard: & Gameboard, stone: Stone) -> isize {
	let range:Vec<usize> = (0..SIZE as usize).collect();

	let cmpt = range
		.iter()
		.flat_map(|y| range
			.iter()
			.map(move |x| eval_around(gameboard.cells, *x as isize, *y as isize, stone) )
			.filter_map(|valid| valid))
		.sum();

	// for y in 0..SIZE {
	// 	for x in 0..SIZE {
	// 		if gameboard.cells[x][y] != Stone::NOPE {
	// 			cmpt += eval_around(gameboard.cells, x as isize, y as isize, stone).unwrap();
	// 		}
	// 	}
	// }

	cmpt
}

pub fn alphabeta(gameboard: & Gameboard, depth: i32, mut alpha: isize, mut beta: isize, noeud_max: bool, stone: Stone, next_stone: Stone) -> (isize, Option<Gameboard>) {
	

	// let mut new_board = gameboard.clone();
	// println!("alphabeta!, board:: ");
	printboard(gameboard);

	// if (game over or depth <= 0)
	if depth <= 0 {
		return (eval(gameboard, stone), None);
	}
	let mut best_board = gameboard.clone();

	// // // NEGA MAX ALPHA BETA? not work ;(
	// for new_board in gameboard.expand(stone) {
	// 	let (mut score, _) = alphabeta(&new_board, depth - 1, -beta, -alpha, !noeud_max, next_stone, stone);
	// 	score = -score;
	// 	if score >= alpha {
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
			let (score, _) = alphabeta(&new_board, depth - 1, alpha, beta, !noeud_max, next_stone, stone);
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
		for new_board in gameboard.expand(stone) {
			let (score, _) = alphabeta(&new_board, depth - 1, alpha, beta, !noeud_max, next_stone, stone);
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
