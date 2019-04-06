use crate::models::gameboard::*;
use std::collections::HashMap;


pub fn dbg_line(mut line : u16) {
	for _ in 0..6 {
		match ((line & 0b11_00_00_00_00_00) >> 10) as u8 {
			WHITE => (print!("x")),
			BLACK => (print!("o")),
			NOPE => (print!("-")),
			_ => (),
		}
		line<<=2;
	}
	print!(" ");
}

	pub fn evale_one_line(l: u64) -> isize {
		let mut value = 0;
		let mut j: isize;
		let mut line = l;
		while line != 0 {
			// dbg_line((line & 0b1111_1111_1111) as u16);
			match (line & 0b11_11_11_11_11_11) as u16 {
				// ......
				0b00_00_00_00_00_00  => {
						j = 10;
				},
				// ....o.
				// ....x.
				0b00_00_00_00_01_00 => {
					// println!(": [00.0] 1 black open");
					j = 10;
					// value -= 1;
				}
				0b00_00_00_00_10_00 => {
					// println!(": [00.1] 1 white open");
					j = 10;
					// value += 1;
				}
				// ...ox.
				// ...xo.
				0b00_00_00_01_10_00 |
				0b00_00_00_10_01_00 => {
					// println!(": [01] ...xo.||...ox.");
					j = 10;
				}
				// _xxxxx
				align5_white if (align5_white & 0b11_11_11_11_11 == 0b10_10_10_10_10) => {
					// println!(": [02]RETURN 10000000 align5 white");
					value += 10000000;
					j = 10;
				},
				// _ooooo
				align5_black if (align5_black & 0b11_11_11_11_11 == 0b01_01_01_01_01) => {
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": [03]RETURN - 10000000 align5 black");
					value -= 10000000;
					j = 10;
				},
				// .oooo.
				0b00_01_01_01_01_00 => {
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": [04]value -= 100000 align4 open black");
					value -= 100000;
						j = 10;
				},
				// .xxxx.
				0b00_10_10_10_10_00 => {
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": [05]value += 100000 align4 open white");
					value += 100000;
						j = 10;
				},
				// ..ooox
				// .o.oox
				// xo.oo.
				// xooo..
				0b00_00_01_01_01_10 | 
				0b00_01_00_01_01_10 | 
				0b10_01_00_01_01_00 | 
				0b10_01_01_01_00_00 => {
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": [06]value -= 100 align3 close black");
					value -= 100;
						j = 10;
				},
				// ..xxxo
				// .x.xxo
				// ox.xx.
				// oxxx..
				0b00_00_10_10_10_01 |
				0b00_10_00_10_10_01 |
				0b01_10_00_10_10_00 |
				0b01_10_10_10_00_00  => {
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": [07]Value += 100 align3 close white");
						value += 100;
						j = 10;
				},
				//xo.ooo
				//xoo.oo
				//xooo.o
				//xoooo.
				//.oooox
				//o.ooo.
				//oo.oo.
				//ooo.o.
				// .ooo.o

				0b10_01_00_01_01_01 |
				0b10_01_01_00_01_01 |
				0b10_01_01_01_00_01 |
				0b10_01_01_01_01_00 |
				0b00_01_01_01_01_10 |
				0b01_00_01_01_01_00 |
				0b01_01_00_01_01_00 |
				0b01_01_01_00_01_00 | 
				0b00_01_01_01_00_01 => {
				// dbg_line((line & 0b1111_1111_1111) as u16);
				// println!(": [08]value -= 10000 align4 close black");
					value -= 10000;
						j = 10;
				},
				//ox.xxx
				//oxx.xx
				//oxxx.x
				//oxxxx.
				//x.xxx.
				//xx.xx.
				//xxx.x.

				// .xxx.x
				0b01_10_00_10_10_10 |
				0b01_10_10_00_10_10 |
				0b01_10_10_10_00_10 |
				0b01_10_10_10_10_00 |
				0b00_10_10_10_10_01 |

				0b10_00_10_10_10_00 |
				0b10_10_00_10_10_00 |
				0b10_10_10_00_10_00 |
				0b00_10_10_10_00_10 => {
				// dbg_line((line & 0b1111_1111_1111) as u16);
				// println!(": [09]value += 10000 align4 close white");
					value += 10000;
						j = 10;
				},
				//_..oo.
				align2black_open if align2black_open & 0b00_11_11_11_11_11 == 0b00_00_00_01_01_00 => {
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": [10]value -= 100 align2 open black");
					value -= 100;
						j = 8;
				},
				//_..xx.
				align2white_open if align2white_open & 0b00_11_11_11_11_11 == 0b00_00_00_10_10_00 => {
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": [11]Value += 100 align2 open white");
					value += 100;
					j = 8;
				},
				// //...xxo
				// //oxx...
				// align2black_close if align2black_close & 0b00_11_11_11_11_11 == 0b00_00_00_01_01_10
				// 			|| align2black_close & 0b11_11_11_11_11_11 == 0b10_01_01_00_00_00 => {
				// 	// dbg_line((line & 0b1111_1111_1111) as u16);
					 // // println!(": [12]value -= 100 align2 close black");
				// 	value -= 50;
				// 		j = 8;
				// },
				// //...oox
				// //xoo...
				// align2white_close if align2white_close & 0b00_11_11_11_11_11 == 0b00_00_00_10_10_01
				// 			|| align2white_close & 0b11_11_11_11_11_11 == 0b01_10_10_00_00_00 => {
				// 	// dbg_line((line & 0b1111_1111_1111) as u16);
					// // println!(": [13]value += 10 align2 close white");
				// 	value += 50;
				// 		j = 8;
				// },
				//_.o.o.
				align2black_hole if align2black_hole & 0b00_11_11_11_11_11 == 0b00_00_01_00_01_00 => {
					// println!(": [14]value -= 10 align2 hole black");
					value -= 10;
					j = 8;
				},
				//_.x.x.
				align2white_hole if align2white_hole & 0b00_11_11_11_11_11 == 0b00_00_10_00_10_00 => {
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": [15]Value += 10 align2 hole white");
					value += 10;
					j = 8;
				},
				// _.ooo.
				align3black if (align3black & 0b00_11_11_11_11_11) == 0b00_00_01_01_01_00 => {

					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": [16]value -= 10000 align3 open black");
					value -= 10000;
						j = 8;

				},
				// _oo.o.
				// _o.oo.
				align3black if (align3black & 0b00_11_11_11_11_11) == 0b00_01_01_00_01_00
							|| (align3black & 0b00_11_11_11_11_11) == 0b00_01_00_01_01_00 => {
					// println!(": [17]value -= 10000 align3 open black");
					value -= 10000;
						j = 8;
				},
				// _.xxx.
				align3white if (align3white & 0b00_11_11_11_11_11) == 0b00_00_10_10_10_00 => {
					// println!(": [18]Value += 1000 align3 open white");
						value += 1000;
						j = 8;
				}
				// _xx.x.
				// _x.xx.
				align3white if (align3white & 0b00_11_11_11_11_11) == 0b00_10_10_00_10_00
							|| (align3white & 0b00_11_11_11_11_11) == 0b00_10_00_10_10_00 => {
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": [19]Value += 1000 align3 open white");
						value += 1000;
						j = 8;
				}

				_ => j = 2,
			}
			line>>=j;
		}
		// map_lines_values.insert(l, value);
		value
	}
// }

fn get_all_diag1(cells: &[u64; SIZE]) -> Vec<u64> {
	let mut vec: Vec<u64> = (4..SIZE).map(|x| down_diago!(cells, x, 0, x, 0)).collect();
	let vec2: Vec<u64> = (1..SIZE-4).map(|x| down_diago!(cells, 0, SIZE - 1 -x, x, SIZE - 1)).collect();
	vec.extend(vec2);
	vec
}

pub fn print_possible_move(possible: &[u32; SIZE]) {

	print!("POSSIBLE MOVE:\n");
	for x in 0..SIZE { print!("{0: <2} ", x) };
	print!("\n");
	for y in 0..SIZE {
		print!("{0: <2} ", y);
		for x in 0..SIZE {
			if possible[x] >> y & 0b1 == 1 {
				print!("x  ");
			}
			else {
				print!(".  ");
			}
		}
		print!("\n");
	}

}

fn get_all_diag2(cells: &[u64; SIZE]) -> Vec<u64> {
	let mut vec: Vec<u64> = (0..SIZE-4).map(|x| up_diago!(cells, 0, SIZE - 1 - x, x, 0)).collect();
	let vec2: Vec<u64> = (1..SIZE-4).map(|y| up_diago!(cells, 0, SIZE -1, 0, y)).collect();
	vec.extend(vec2);
	vec
}

    pub fn eval(state: &Gameboard, actual_stone: u8, depth: u8, map_board_values: &mut HashMap<[u64; SIZE], isize>, player_stone: u8) -> isize {

		let mut score = if state.black_captures >= 10 {
				-10000000
		} else if state.white_captures >= 10 {
				10000000
		} else if map_board_values.contains_key(&state.cells) {
				*map_board_values.get(&state.cells).unwrap()
		} else {
			let mut all: Vec<u64> = (0..SIZE).map(|y| line_horizontal!(state.cells, 0, SIZE - 1, y as usize)).collect();
			let all_verti: Vec<u64> = (0..SIZE).map(|x| line_vertical!(state.cells[x as usize], 0 , SIZE -1)).collect();
			let all_diag_1 = get_all_diag1(&state.cells);
			let all_diag_2 = get_all_diag2(&state.cells);

			all.extend(all_verti);
			all.extend(all_diag_1);
			all.extend(all_diag_2);
			all.retain(|&elem| elem != 0);
			let value = all.iter().map(|&e| evale_one_line(e)).sum();
			map_board_values.insert(state.cells, value);
			value
		};
		score += (state.white_captures as isize * state.white_captures as isize * 100) - (state.black_captures as isize * state.black_captures as isize * 100);
		if player_stone == BLACK {
			score = -score;
		}
		score *= depth as isize + 1;
		// printboard!(state.cells);
		// print_possible_move(&state.possible_moves);
		if actual_stone == player_stone {
			// println!("actual_stone == player_stone: score: {}\n\n------------------\n\n", score);
			// println!("\nEVAL: {} (depth: {})", score, depth);
			// println!("\n---------------------\n\n");

			score// + (depth as isize * 10000) //- (state.white_captures * state.white_captures * 10) as isize + (state.black_captures  * state.black_captures * 10) as isize
		} else {
			// println!("\nEVAL: {} (depth: {})", -score, depth);
			// println!("\n---------------------\n\n");
			-score// + (depth as isize * 10000) //+ (state.white_captures * state.white_captures * 10) as isize - (state.black_captures  * state.black_captures * 10) as isize
			}
	}
