use crate::models::gameboard::{Gameboard, Stone, SIZE, AlignType};

pub fn algo(gameboard: &mut Gameboard, current_stone: Stone) -> (isize, Option<Gameboard>) {
	let alpha = std::isize::MIN;
	let beta = std::isize::MAX;

	let next_stone: Stone;
	if current_stone == Stone::BLACK {
		next_stone = Stone::WHITE;
	} else {
		next_stone = Stone::BLACK;
	}
	// gameboard.set_window_actives_cells();
	alphabeta(gameboard, 1, alpha + 1, beta, true, current_stone, next_stone)
}

pub fn len_of_one_align(align: &(AlignType, usize, usize, usize, usize , bool, bool)) -> isize {

	match align.0 {
		AlignType::HORIZONTAL => (align.3 - align.1) as isize,
		AlignType::VERTICAL => (align.4 - align.2) as isize,
		AlignType::DIAGONAL1 => (align.3 - align.1) as isize,
		AlignType::DIAGONAL2 => (align.3 - align.1) as isize,
	}
}

pub fn is_left_open(align: &(AlignType, usize, usize, usize, usize , bool, bool)) -> bool {
	align.5
}

pub fn is_right_open(align: &(AlignType, usize, usize, usize, usize , bool, bool)) -> bool {
	align.6
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

pub fn eval_one_direction(cells: [[Stone; SIZE]; SIZE], x_orig: isize, y_orig: isize, stone: Stone, after: [(isize, isize); 4], win: [usize; 4]) -> isize {
	// println!("eval_one_direction: stone:: {:?}",stone);
	let mut cmpt = 0;
	// let other_stone;
	// if stone == Stone::BLACK {
	// 	 other_stone = Stone::WHITE;
	// } else {
	// 	 other_stone = Stone::BLACK;
	// }
	// let not_align_color;
	// 	if align_color == Stone::BLACK {
	// 	 not_align_color = Stone::WHITE;
	// } else {
	// 	 not_align_color = Stone::BLACK;
	// }
	// let mut current_stone;// = cells[x_orig as usize][y_orig as usize];
	let align_color = cells[x_orig as usize][y_orig as usize];

	let mut align_len = 1;
	for (x, y) in after.iter().filter(|(x, y)| *x >= win[0] as isize && *y >= win[1] as isize && *x < win[2] as isize && *y < win[3] as isize) {
		let current_stone = cells[*x as usize][*y as usize];
		if current_stone == align_color {
			cmpt = cmpt + (align_len * align_len + 5);
			align_len += 1;
		}
		else { break; }
	}
	if align_len > 5 {
		cmpt += 1000;
	}
	if align_color != stone {
		-cmpt
	} else {
		cmpt
	}
}


pub fn eval_all_directions(cells: [[Stone; SIZE]; SIZE], x: isize, y: isize, stone: Stone, win: [usize; 4]) -> Option<isize> {

	if cells[x as usize][y as usize] == Stone::NOPE { return None;}
	let after_horizontal: [(isize, isize); 4] = [(x + 1, y),
		(x + 2, y),
		(x + 3, y),
		(x + 4, y)];
	let after_vertical: [(isize, isize); 4] = [(x, y + 1),
		(x, y + 2),
		(x, y + 3),
		(x, y + 4)];
	let after_diag_1: [(isize, isize); 4] = [(x + 1, y + 1),
		(x + 2, y + 2),
		(x + 3, y + 3),
		(x + 4, y + 4)];
	let after_diag_2: [(isize, isize); 4] = [(x + 1, y - 1),
		(x + 2, y - 2),
		(x + 3, y - 3),
		(x + 4, y - 4)];
	let afters = [after_horizontal, after_vertical, after_diag_1, after_diag_2];

	let value = afters.iter().map(|after| eval_one_direction(cells, x, y, stone, *after, win)).sum();
	Some(value)
}


pub fn eval(gameboard: & Gameboard, stone: Stone) -> isize {
	// let range_h:Vec<usize> = (gameboard.win[0]..gameboard.win[2] as usize).collect();
	// let range_v:Vec<usize> = (gameboard.win[1]..gameboard.win[3] as usize).collect();

	println!("---------------------------------------\n\t\tALIGN WHITE: gameboard.align_list_white: len: {}", gameboard.align_list_white.len());
	dbg!(&gameboard.align_list_white);
	println!("---------------------------------------\n\t\tALIGN BLACK: gameboard.align_list_white: len: {}", gameboard.align_list_white.len());
	dbg!(&gameboard.align_list_black);

	let mut value: isize = 0;
	if stone == Stone::WHITE {
		for align in &gameboard.align_list_white {
			value += len_of_one_align(align);
			if is_left_open(align) {
				value += 1;
			}
			if is_right_open(align) {
				value += 1;
			}
		}
		for align in &gameboard.align_list_black {
			value -= len_of_one_align(align);
			if is_left_open(align) {
				value += 1;
			}
			if is_right_open(align) {
				value += 1;
			}
		}
	}
	else {
		for align in &gameboard.align_list_black {
			value += len_of_one_align(align);
			if is_left_open(align) {
				value += 1;
			}
			if is_right_open(align) {
				value += 1;
			}
		}
		for align in &gameboard.align_list_white {
			value -= len_of_one_align(align);
			if is_left_open(align) {
				value += 1;
			}
			if is_right_open(align) {
				value += 1;
			}
		}
	}

	// let value:isize = range_v.iter()
	// 	.flat_map(|y| range_h
	// 		.iter()
	// 		.map(move |x| eval_all_directions(gameboard.cells, *x as isize, *y as isize, stone, gameboard.win) )
	// 		.filter_map(|some| some))
	// 	.sum();
	value
}

pub fn alphabeta(gameboard: & Gameboard, depth: i32, mut alpha: isize, mut beta: isize, noeud_max: bool, stone: Stone, next_stone: Stone) -> (isize, Option<Gameboard>) {
	if depth <= 0 || gameboard.max_align() > 4 {
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

			// new_board.set_align(gameboard);
			
			let (score, _) = alphabeta(&new_board, depth - 1, alpha, beta, !noeud_max, stone, next_stone);
			if score > alpha {
				alpha = score;
				// println!("_________ new board: for MAX (stone: {:?}), value: {}, BOARD::", stone, score);
				// printboard(&new_board);
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
				// println!("_________ new better board: for MIN (stone: {:?}), value: {}, BOARD::", stone, score);
				// printboard(&new_board);
				beta = score;
				best_board = new_board.clone();
				if alpha >= beta {
					break ;
				}
			}
		}
	// println!("_________ better board: for MIN (stone: {:?}), value: {}, BOARD::", stone, beta);
	// printboard(&best_board);
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