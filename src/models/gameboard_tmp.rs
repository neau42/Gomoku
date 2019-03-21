//! Gameboard

/// Size of game board.
pub const SIZE: usize = 19;

#[derive(Debug,PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Stone {
    BLACK,
    WHITE,
	NOPE,
}

impl Stone {
    pub fn switch(&mut self) {
        *self = match *self {
            Stone::BLACK => Stone::WHITE,
            Stone::WHITE => Stone::BLACK,
            _ => return,
        }
    }

	pub fn opposant(self) -> Stone {
        match self {
            Stone::BLACK => Stone::WHITE,
            _ => Stone::BLACK,
        }
    }
}

/// Stores game board information.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Gameboard {
	pub size: usize,
    pub cells: [[Stone; SIZE]; SIZE],
	pub value: isize,
	pub white_captures : usize,
	pub black_captures : usize,
	pub possible_moves: Vec<(usize, usize)>,
	// pub possible_moves: [[bool; SIZE]; SIZE],
	pub selected_move: Option<(usize, usize)>,
}

/// Creates a new game board.
impl Gameboard {
	pub fn new() -> Gameboard {
		Gameboard {
			size: SIZE,
			cells: [[Stone::NOPE; SIZE]; SIZE],
			value: 0,
			white_captures: 0,
			black_captures: 0,
			// possible_moves: [[false; SIZE]; SIZE],
			possible_moves: Vec::new(),
			selected_move: None,
		}
	}
}

pub fn eval_value(cells: &[[Stone; SIZE]; SIZE], x_orig: isize, y_orig: isize, stone: Stone) -> (isize, usize) {
	let mut value = 0;
	let mut nb_captures = 0;
	let x_min = (x_orig - 5).max(0);
	let x_max = (x_orig + 5).min(SIZE as isize - 1);
	let y_min = (y_orig - 5).max(0);
	let y_max = (y_orig + 5).min(SIZE as isize - 1);

	let horizontal: Vec<Stone> = (x_min..=x_max).map(|x| cells[x as usize][y_orig as usize]).collect();
	let vertical: Vec<Stone> = (y_min..=y_max).map(|y| cells[x_orig as usize][y as usize]).collect();

	let len_origin_min = (y_orig - y_min).min(x_orig - x_min);
	let len_origin_max = (y_max - y_orig).min(x_max - x_orig);

	let diag1: Vec<Stone> = ((x_orig-len_origin_min)..=(x_orig + len_origin_max)).enumerate()
	.map(|(index, x)| cells[x as usize][y_orig as usize - len_origin_min as usize + index])
	.collect();

	let len_origin_min = (y_max - y_orig).min(x_orig - x_min);
	let len_origin_max = (y_orig - y_min).min(x_max - x_orig);
	let diag2: Vec<Stone> = ((x_orig-len_origin_min)..=(x_orig + len_origin_max)).enumerate()
	.map(|(index, x)| cells[x as usize][y_orig as usize + len_origin_min as usize - index])
	.collect();

	let list = [horizontal, vertical, diag1, diag2];
	for elem in &list {
		let eval = eval_line(&elem, stone, stone.opposant());
		value += eval.0;
		if eval.1 > 0 {
			nb_captures += eval.1;
		}
	}
	(value, nb_captures)
}

pub fn analyze_slice_of_6(slice: &[Stone], current_stone: Stone, other_stone: Stone) -> (isize, usize) {

	match slice {
		test if test[0] == current_stone && test[0] == test[1] && test[0] == test[2] && test[0] == test[3] && test[0] == test[4] => (42, 0),
		[_, s1, s2, s3, s4, _] if *s1 == current_stone && *s2 == other_stone && s2 == s3 && s1 == s4 => (2,1),// capture
		[s1, s2, s3, s4, _, _] if *s1 == current_stone && *s2 == other_stone && s2 == s3 && s1 == s4 => (2,1),// capture
		[_, _, s1, s2, s3, s4] if *s1 == current_stone && *s2 == other_stone && s2 == s3 && s1 == s4 => (2,1),// capture
		[Stone::NOPE, s1, s2, s3, s4, Stone::NOPE] => {
			match (s1,s2,s3,s4) {
				(s1, s2, s3, s4) if *s1 == current_stone && s1 == s2 && s1 == s3 && s1 == s4 => (4, 0),		// align 4
				(s1, s2, s3, Stone::NOPE) if *s1 == current_stone && s1 == s2 && s1 == s3 => (3, 0),		// align 3
				(s1, s2, Stone::NOPE, s3) if *s1 == current_stone && s1 == s2 && s1 == s3 => (3, 0),		// align 3
				(s1, Stone::NOPE, s2, s3) if *s1 == current_stone && s1 == s2 && s1 == s3 => (3, 0),		// align 3
				(s1, s2, Stone::NOPE, Stone::NOPE) if *s1 == current_stone && s1 == s2 => (1, 0),			// align 3
				(s1, s2, Stone::NOPE, Stone::NOPE) if *s1 == current_stone && s1 == s2 => (1, 0),			// align 2
				_ => (0, 0),
			}
		}
		_ => (0, 0),
	}
}

pub fn eval_line(slice: &[Stone], current_stone: Stone, other_stone: Stone) -> (isize, usize) {
	let mut value = 0;
	let mut nb_captures = 0;
	let mut len = slice.len();
	if len < 5 { return (0, 0); }

	while len > 6 {
		let eval = analyze_slice_of_6(&slice[len-6..len], current_stone, other_stone);
		value+=eval.0;
		nb_captures+=eval.1;
		// println!("value: {}", value);
		len -= 1;
	}
	if len > 0 {
		let eval = analyze_slice_of_6(&slice[0..len], current_stone, other_stone);
		value+=eval.0;
		nb_captures+=eval.1;
		// println!("value: {}", value);
	}
	// println!("value of line: {}", value);
	(value, nb_captures)
}

impl Gameboard {

	// pub fn update_possible_move(&mut self, x: usize, y: usize) {
    //     let directions: [(isize, isize); 8] = [(0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1)];
    //     directions.iter().for_each(|(tmp_x, tmp_y)| {
	// 		let tmp_x = *tmp_x + x as isize;
	// 		let tmp_y = *tmp_y + y as isize;
	// 		if tmp_x < 0 || tmp_x >= SIZE as isize || tmp_y < 0 || tmp_y >= SIZE as isize {
	// 			return;
	// 		}
	// 		if self.cells[tmp_x as usize][tmp_y as usize] == Stone::NOPE {
	// 			self.possible_moves[tmp_x as usize][tmp_y as usize] = true;
	// 		}
	// 	})
	// }

	pub fn update_possible_move(&self, x: isize, y: isize) -> Vec<(usize, usize)>{
		let min_x = (x - 1).max(0) as usize;
		let min_y = (y - 1).max(0) as usize;
		let max_x = (x + 1).min(self.size as isize - 1) as usize;
		let max_y = (y + 1).min(self.size as isize - 1) as usize;

		let x = x as usize;
		let y = y as usize;
		let moves = [(min_x, y), (min_x, min_y), (min_x, max_y), (max_x, y), (max_x, min_y), (max_x, max_y), (x, min_y), (x, max_y)];
		moves
			.into_iter()
			.filter(|new_move| {
				self.cells[new_move.0][new_move.1] == Stone::NOPE && !self.possible_moves.contains(*new_move)
			}).map(|new_move| *new_move).collect()
	}

	// pub fn next_move(&mut self, mut starting_x: usize, mut starting_y: usize) {
    //     if starting_x >= SIZE {
    //         starting_x = 0;
    //         starting_y += 1;
    //         if starting_y >= SIZE {
    //             self.selected_move = None;
    //             return;
    //         }
    //     }
    //     self.selected_move = None;
	// 	(0..SIZE)
	// 		.filter(|y| *y >= starting_y)
	// 		.any(|y| (0..SIZE)
	// 			.filter(|x| y > starting_y || *x >= starting_x)
	// 			.any(|x| {
	// 				if self.possible_moves[x][y] && self.cells[x][y] == Stone::NOPE {
    //                     self.selected_move = Some((x, y));
	// 					return true;
	// 				}
	// 				false
	// 			})
    //     );
	// }

	// pub fn expand(&self) -> Vec<(usize, usize)> {
	// 	(0..SIZE)
	// 	.flat_map(|y| {
	// 		(0..SIZE)
	// 		.filter(move |&x| self.possible_moves[x][y as usize] && self.cells[x][y as usize] == Stone::NOPE)
	// 		.map(move |x| (x, y))
	// 	})
	// 	.collect()
	// }
}

impl Gameboard {
	pub fn apply_capture(&mut self, x: usize, y: usize, stone: Stone, other_stone: Stone, mut nb_capture: usize) {
        let directions: [(isize, isize); 8] = [(0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1)];

		while nb_capture > 0 {
			directions.iter().any(|(tmp_x, tmp_y)| {
				let mut xy1 = None;
				let mut xy2 = None;
				(1..=3 as isize).all(|i| {
					let tmp_x = *tmp_x  * i + x as isize;
					let tmp_y = *tmp_y * i + y as isize;
					if tmp_x < 0 || tmp_x >= SIZE as isize || tmp_y < 0 || tmp_y >= SIZE as isize {
						return false;
					}
					let tmp_stone = self.cells[tmp_x as usize][tmp_y as usize];
						if i <= 2 {
							if tmp_stone == other_stone {
								if i == 1 {
									xy1 = Some((tmp_x, tmp_y));
								} else {
									xy2 = Some((tmp_x, tmp_y));
								}
								true 
							} else { false }
						}
						else {
							if tmp_stone == stone {
								self.cells[xy1.unwrap().0 as usize][xy1.unwrap().1 as usize] = Stone::NOPE;
								self.cells[xy2.unwrap().0 as usize][xy2.unwrap().1 as usize] = Stone::NOPE;
								true
							} else { false }
					}
				})
			});
			nb_capture -= 1;
		}
	}

	// pub fn make_move(&mut self, x: usize, y: usize, stone: Stone) -> bool {
	// 	if self.cells[x][y] == Stone::NOPE && !self.check_double_tree(x, y, stone) {
	// 		let other_stone = match stone {
	// 				Stone::WHITE => Stone::BLACK,
	// 				Stone::BLACK => Stone::WHITE,
	// 				_ => Stone::WHITE,
	// 			};
	// 		self.cells[x][y] = stone;
	// 		self.update_possible_move(x, y);
	// 		// println!("\n-------------------");
	// 		// self.printboard();
	// 		// println!("x: {}, y: {}", x, y);
	// 		// let val = eval_value(&self.cells, x as isize, y as isize, stone);
	// 		self.value = 0;
	// 		// self.value = val.0;
	// 		// if val.1 > 0 {

	// 		// 	// self.apply_capture(x, y, stone, other_stone, val.1); //				       APPLY CAPTURE !!!!!!!!!
	// 		// 	match stone {
	// 		// 		Stone::WHITE => self.white_captures += val.1,
	// 		// 		Stone::BLACK => self.black_captures += val.1,
	// 		// 		_ => (),
	// 		// 	}
	// 		// }
	// 		return true;
    //     }
    //     false
    // }

	 pub fn make_move(&mut self, x: usize, y: usize, stone: Stone) -> bool {
		if self.cells[x][y] == Stone::NOPE {
            self.cells[x][y] = stone;
            let actual_move = (x, y);
            // println!("avant ?{}? {}|{}", self.possible_moves.len(), x, y);
            // dbg!(&self.possible_moves);
            if let Some(index) = self.possible_moves.iter().position(|&possible_move| possible_move == actual_move) {
                // println!("index = {}", index);
				self.possible_moves.remove(index);
			}
            self.possible_moves.extend(self.update_possible_move(x as isize, y as isize));
            // println!("apres ?{}?", self.possible_moves.len());
            // dbg!(&self.possible_moves);
            return true
        }
        false
    }
	
	pub fn printboard(&self) {
		print!("BOARD: \n   ");
		for x in 0..SIZE {
			print!("{0: <2} ", x);
		}
		println!();

		for y in 0..SIZE {
			print!("{0: <2} ", y);
			for x in 0..SIZE {
				match self.cells[x][y] {
					Stone::WHITE => print!("W  "),
					Stone::BLACK => print!("B  "),
					_ => print!(".  ")
				}
			}
			println!();
		}
	}

    pub fn unmake_move(&mut self, x: usize, y: usize) {
        self.cells[x][y] = Stone::NOPE;
    }

}

impl Gameboard {
    // True if capture is possible
    pub fn check_capture(&self, x: usize, y: usize, actual_stone: Stone) -> bool {
        let directions: [(isize, isize); 8] = [(0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1)];

        directions.iter().any(|(tmp_x, tmp_y)| {
            (1..=3 as isize).all(|i| {
                let tmp_x = *tmp_x  * i + x as isize;
                let tmp_y = *tmp_y * i + y as isize;
                if tmp_x < 0 || tmp_x >= self.size as isize || tmp_y < 0 || tmp_y >= self.size as isize {
                    return false;
                }
                let tmp_stone = self.cells[tmp_x as usize][tmp_y as usize];
                match i {
                    1 | 2 => tmp_stone != actual_stone && tmp_stone != Stone::NOPE,
                    _ => tmp_stone == actual_stone,
                }
            })
        })
	}

	pub fn check_double_tree(&self, x: usize, y: usize, actual_stone: Stone) -> bool {
        // let directions: [(isize, isize); 4] = [(0,1), (1,0), (1,1), (1,-1)];
        // let closure = |tmp_x: isize, tmp_y: isize| -> Vec<Stone> {
        //     (1..=5 as isize).filter_map(|i| {
        //         let tmp_x = tmp_x  * i + x as isize;
        //         let tmp_y = tmp_y * i + y as isize;
        //         if tmp_x < 0 || tmp_x >= self.size as isize || tmp_y < 0 || tmp_y >= self.size as isize {
        //             return None;
        //         }
        //         Some(self.cells[tmp_x as usize][tmp_y as usize])
        //     }).collect()
        // };
        
        // let nbr_tree = directions.iter().fold(0, |nbr_tree, (tmp_x, tmp_y)| {
        //     let right_side = closure(*tmp_x, *tmp_y);
        //     let mut left_side = closure(tmp_x * -1, tmp_y * -1);
        //     left_side.reverse();
        //     let line = [&left_side[..], &vec![actual_stone][..], &right_side[..]].concat();
        //     let len = line.len();
        //     if len < 6 {
        //         return nbr_tree;
        //     }
        //     let is_tree: bool = (0..=(len - 6)).any(|i| {
        //         line[i] == Stone::NOPE
        //         && line[i + 5] == Stone::NOPE
        //         && line[(i + 1)..(i + 5)].iter()
        //         .fold(0, |sum, stone| {
        //             match *stone {
        //                 otherstone if otherstone == actual_stone => sum + 1,
        //                 Stone::NOPE => sum + 2,
        //                 _ => sum + 3,
        //             }
        //         }) == 5
        //     });
        //     if is_tree {
        //         nbr_tree + 1
        //     }
        //     else {
        //         nbr_tree
        //     }
        // });
        // // println!("nbr_tree = {}", nbr_tree);
        // nbr_tree >= 2
		false
	}
}
