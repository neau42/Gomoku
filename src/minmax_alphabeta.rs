use crate::models::gameboard::{Gameboard, Stone, SIZE};

// const SIZE: usize = 19;

pub fn algo(gameboard: &mut Gameboard) {
	let alpha = std::isize::MAX;
	let beta = std::isize::MIN;

	for x in 0..SIZE {
		for y in 0..SIZE {
			match gameboard.cells[x][y] {
				Stone::WHITE => gameboard.white_stone[x][y] = true,
				Stone::BLACK => gameboard.black_stone[x][y] = true,
				_ => (),
			}
		}
	}
	println!("WHITE STONE:");
	for x in 0..SIZE {
		for y in 0..SIZE {
			if gameboard.white_stone[x][y] == true {
				print!("w ")
			} else {
				print!(". ")
			}
		}
		println!("");
	}
	println!("BLACK STONE:");
	for x in 0..SIZE {
		for y in 0..SIZE {
			if gameboard.black_stone[x][y] == true {
				print!("b ")
			} else {
				print!(". ")
			}
		}
		println!("");
	}
}
		
pub fn eval(_gameboard: &mut Gameboard) -> isize {
	0
}

pub fn alphabeta(gameboard: &mut Gameboard, depth: i32 ,alpha:  i32, beta:  i32, white_stone: &[[bool; SIZE]; SIZE], black_stone: &[[bool; SIZE]; SIZE]) -> isize {
//    if (game over or depth <= 0)
	if (depth <= 0) {
		return eval(gameboard);
	}

// //    move bestmove ;
	for x in 0..SIZE {
		for y in 0..SIZE {
			if gameboard.cells[x][y] != Stone::NOPE {
				

			}
		}
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
	0
}