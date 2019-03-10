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
	println!("current_stone: {:?}", current_stone);
	alphabeta(gameboard, 1, alpha + 1, beta, true, current_stone, next_stone)
}


pub fn printboard(gameboard: & Gameboard) {
	println!("BOARD: ");
	for y in 0..SIZE {
		for x in 0..SIZE {
			match gameboard.cells[x][y] {
				Stone::WHITE => print!("W "),
				Stone::BLACK => print!("B "),
				_ => print!(". ")
			}
		}
		println!("");
	}
}

pub fn eval_one_direction(cells: [[Stone; SIZE]; SIZE], x_orig: isize, y_orig: isize, stone: Stone, after: [(isize, isize); 4]) -> isize {
	println!("eval_one_direction: stone:: {:?}",stone);
	let mut cmpt = 1;
	let other_stone;
	let mut current_stone = cells[x_orig as usize][y_orig as usize];
	if stone == Stone::BLACK {
		 other_stone = Stone::WHITE;
	} else {
		 other_stone = Stone::BLACK;
	}
	let align_stone = cells[x_orig as usize][y_orig as usize];
	let not_align_stone;
		if align_stone == Stone::BLACK {
		 not_align_stone = Stone::WHITE;
	} else {
		 not_align_stone = Stone::BLACK;
	}

	for (x, y) in after.iter().filter(|(x, y)| *x >= 0 && *y >= 0 && *x < 19 && *y < 19) {
		let prev_stone = current_stone;
		current_stone = cells[*x as usize][*y as usize];
		if current_stone == stone {
			println!("=========> x:{},y:{} CMPT+=3!", *x, *y);
			cmpt = cmpt * 2 + 3;
		// } else if current_stone == other_stone {
		// 	cmpt = cmpt * 2 - 3;// ((x-x_orig).abs().max((y-y_orig).abs()));

		} else if prev_stone == Stone::NOPE && current_stone == Stone::NOPE { break; }

		 if current_stone == not_align_stone { break; }
	}
	cmpt
}


pub fn eval_all_directions(cells: [[Stone; SIZE]; SIZE], x: isize, y: isize, stone: Stone) -> Option<isize> {

	if cells[x as usize][y as usize] == Stone::NOPE { return None;}
	let after_horizontal: [(isize, isize); 4] = [(x+1,y),
		(x+2, y),
		(x+3, y),
		(x+4, y)];
	let after_vertical: [(isize, isize); 4] = [(x,y+1),
		(x, y+2),
		(x, y+3),
		(x, y+4)];
	let after_diag_1: [(isize, isize); 4] = [(x+1,y+1),
		(x+2, y+2),
		(x+3, y+3),
		(x+4, y+4)];
	let after_diag_2: [(isize, isize); 4] = [(x+1,y-1),
		(x+2, y-2),
		(x+3, y-3),
		(x+4, y-4)];
	let afters = [after_horizontal, after_vertical, after_diag_1, after_diag_2];

	let value = afters.iter().map(|after| eval_one_direction(cells, x, y, stone, *after)).sum();

	Some(value)
}


pub fn eval(gameboard: & Gameboard, stone: Stone) -> isize {
	let range:Vec<usize> = (0..SIZE as usize).collect();
	let  value:isize = range.iter()
		.flat_map(|y| range
			.iter()
			.map(move |x| eval_all_directions(gameboard.cells, *x as isize, *y as isize, stone) )
			.filter_map(|some| some))
		.sum();


	if value > 0 {
		println!("~~~~~~~~~ EVAL: MAP: ");
		printboard(gameboard);
		println!("~~~~~~~~~ EVAL: VALUE: {} ", value);
	}
	value
}

pub fn alphabeta(gameboard: & Gameboard, depth: i32, mut alpha: isize, mut beta: isize, noeud_max: bool, stone: Stone, next_stone: Stone) -> (isize, Option<Gameboard>) {
	// println!("__alphabeta:");
	// printboard(gameboard);
	// println!("stone: {:?}", stone);

	if depth <= 0 {
		return (eval(gameboard, next_stone), None);
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
				println!("_________ new better board: for MAX (stone: {:?}) ", stone);
				printboard(&new_board);
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
				println!("_________ new better board: for MIN (stone: {:?}) ", stone);
				printboard(&new_board);
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

// pub fn eval_around(cells: [[Stone; SIZE]; SIZE], x: isize, y: isize, stone: Stone) -> Option<isize> {
// 	if cells[x as usize][y as usize] == Stone::NOPE { return None;}
// 	// println!("eval_around: x{}, y{} => {:?}", x, y, cells[x as usize][y as usize]);
// 	let mut cmpt = 0;
// 	let other_stone;
// 	let around: [(isize, isize); 8] = [(x,y+1),
// 		(x+1, y+1),
// 		(x+1, y),
// 		(x+1, y -1),
// 		(x, y -1),
// 		(x-1, y-1),
// 		(x-1, y),
// 		(x-1, y+1)];

// 	if stone == Stone::BLACK {
// 		 other_stone = Stone::WHITE;
// 	} else {
// 		 other_stone = Stone::BLACK;
// 	}
// 	for (x, y) in around.iter().filter(|(x, y)| *x >= 0 && *y >= 0 && *x < 19 && *y < 19) {
// 				if cells[*x as usize][*y as usize] == stone {
// 					cmpt = (3 + cmpt);
// 				} else if cells[*x as usize][*y as usize] == other_stone {
// 					cmpt = (1 + cmpt);
// 				}
// 	}
// 	Some(cmpt)
// }

// pub fn eval_horizontal(cells: [[Stone; SIZE]; SIZE], x: isize, y: isize, stone: Stone) -> Option<isize> {

// 	if cells[x as usize][y as usize] == Stone::NOPE { return None;}
// 	let mut cmpt = 0;
// 	let after: [(isize, isize); 4] = [(x+1,y),
// 		(x+2, y),
// 		(x+3, y),
// 		(x+4, y)];
// 	let other_stone;
// 	let mut current_stone;

// 	if stone == Stone::BLACK {
// 		 other_stone = Stone::WHITE;
// 	} else {
// 		 other_stone = Stone::BLACK;
// 	}
// 	let align_stone = cells[x as usize][y as usize];
// 	for (x, y) in after.iter().filter(|(x, y)| *x >= 0 && *y >= 0 && *x < 19 && *y < 19) {
// 		current_stone = cells[*x as usize][*y as usize];
// 		if current_stone == stone {
// 			cmpt = cmpt + 1;
// 		} else if current_stone == other_stone {
// 			cmpt = cmpt - 1;
// 		} 
// 		if current_stone != align_stone {break;}
// 	}
// 	// if cmpt > 0 {
// 	// 	println!("eval_horizontal: x:{}, y:{}", x, y);
// 	// 	println!("value: {}", cmpt);
// 	// }
// 	Some(cmpt)
// }
// pub fn eval_vertical(cells: [[Stone; SIZE]; SIZE], x: isize, y: isize, stone: Stone) -> Option<isize> {

// 	if cells[x as usize][y as usize] == Stone::NOPE { return None;}
// 	let mut cmpt = 0;
// 	let after: [(isize, isize); 4] = [(x,y+1),
// 		(x, y+2),
// 		(x, y+3),
// 		(x, y+4)];

// 		let other_stone;
// 	let mut current_stone;

// 	if stone == Stone::BLACK {
// 		 other_stone = Stone::WHITE;
// 	} else {
// 		 other_stone = Stone::BLACK;
// 	}
// 	let align_stone = cells[x as usize][y as usize];
// 	for (x, y) in after.iter().filter(|(x, y)| *x >= 0 && *y >= 0 && *x < 19 && *y < 19) {
// 		current_stone = cells[*x as usize][*y as usize];
// 		if current_stone == stone {
// 			cmpt = cmpt + 1;
// 		} else if current_stone == other_stone {
// 			cmpt = cmpt - 1;
// 		} 
// 		if current_stone != align_stone {break;}
// 	}
// 	// if cmpt > 0 {
// 	// 	// println!("eval_vertical: x:{}, y:{}", x, y);
// 	// 	// println!("value: {}", cmpt);
// 	// }
// 	Some(cmpt)
// }
// pub fn eval_diag_1(cells: [[Stone; SIZE]; SIZE], x: isize, y: isize, stone: Stone) -> Option<isize> {

// 	if cells[x as usize][y as usize] == Stone::NOPE { return None;}
// 	let mut cmpt = 0;
// 	let after: [(isize, isize); 4] = [(x+1,y+1),
// 		(x+2, y+2),
// 		(x+3, y+3),
// 		(x+4, y+4)];
// 		let other_stone;
// 	let mut current_stone;

// 	if stone == Stone::BLACK {
// 		 other_stone = Stone::WHITE;
// 	} else {
// 		 other_stone = Stone::BLACK;
// 	}
// 	let align_stone = cells[x as usize][y as usize];
// 	for (x, y) in after.iter().filter(|(x, y)| *x >= 0 && *y >= 0 && *x < 19 && *y < 19) {
// 		current_stone = cells[*x as usize][*y as usize];
// 		if current_stone == stone {
// 			cmpt = cmpt + 1;
// 		} else if current_stone == other_stone {
// 			cmpt = cmpt - 1;
// 		} 
// 		if current_stone != align_stone {break;}
// 	}
// 	// if cmpt > 0 {
// 	// 	// println!("eval_diag_1: x:{}, y:{}", x, y);
// 	// 	// println!("value: {}", cmpt);
// 	// }
// 	Some(cmpt)
// }

// pub fn eval_diag_2(cells: [[Stone; SIZE]; SIZE], x: isize, y: isize, stone: Stone) -> Option<isize> {

// 	if cells[x as usize][y as usize] == Stone::NOPE { return None;}
// 	let mut cmpt = 0;
// 	let after: [(isize, isize); 4] = [(x+1,y-1),
// 		(x+2, y-2),
// 		(x+3, y-3),
// 		(x+4, y-4)];

// 		let other_stone;
// 	let mut current_stone;

// 	if stone == Stone::BLACK {
// 		 other_stone = Stone::WHITE;
// 	} else {
// 		 other_stone = Stone::BLACK;
// 	}
// 	let align_stone = cells[x as usize][y as usize];
// 	for (x, y) in after.iter().filter(|(x, y)| *x >= 0 && *y >= 0 && *x < 19 && *y < 19) {
// 		current_stone = cells[*x as usize][*y as usize];
// 		if current_stone == stone {
// 			cmpt = cmpt + 1;
// 		} else if current_stone == other_stone {
// 			cmpt = cmpt - 1;
// 		} 
// 		if current_stone != align_stone {break;}
// 	}
// 	Some(cmpt)
// }