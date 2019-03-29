use crate::models::gameboard::*;
use std::collections::HashMap;


pub fn dbg_line(mut line : u16) {
	for _ in 0..6 {
		match ((line & 0b11_00_00_00_00_00) >> 10) as u8 {
			WHITE => (print!("w")),
			BLACK => (print!("b")),
			NOPE => (print!("-")),
			_ => (),
		}
		line<<=2;
	}
	print!(" ");
}

	pub fn evale_one_line(l: u64) -> isize {
		// if map_lines_values.contains_key(&l) {
		// 	// println!("({:#066b}) value: {}, map len: {}",&l, *map_lines_values.get(&l).unwrap(), map_lines_values.len());
		// 	return *map_lines_values.get(&l).unwrap();
		// }

		let mut value = 0;
		let mut j: isize;
		let mut line = l;
		while line != 0 {
			// dbg_line((line & 0b1111_1111_1111) as u16);
			match (line & 0b11_11_11_11_11_11) as u16 {
				0b00_00_00_00_00_00 => {  // ALIGN NULL
						j = 10;
				},
				0b00_10_00_00_00_00 | 0b00_01_00_00_00_00 => {  // ALIGN 1
						j = 10;
				},

				0b00_00_01_01_01_00 | 0b00_01_00_01_01_00  => { //+align3 open
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": value += 1000 align3 open");
					value -= 1000;
						j = 10;
				},
				0b00_10_00_10_10_00 | 0b00_00_10_10_10_00 => {//-align3 open
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": Value -= 1000 align3 open");
						value += 1000;
						j = 10;
				}
				0b00_01_01_01_01_00 | 0b01_00_01_01_01_00 | 0b01_01_00_01_01_00 | 0b01_01_01_00_01_00 => { //align4 open plus
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": value -= 10000 align4 open");
					value -= 10000;
						j = 10;
				},
				0b00_10_10_10_10_00 | 0b10_00_10_10_10_00 | 0b10_10_00_10_10_00 | 0b10_10_10_00_10_00 => { //align4 open moins
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": value += 10000 align4 open");
					value += 10000;
						j = 10;
				},
				0b00_00_01_01_01_10 | 0b00_01_00_01_01_10 | 0b10_01_00_01_01_00 | 0b10_01_01_01_00_00 => { //align3 close plus
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": value -= 100 align3_close");
					value -= 100;
						j = 10;
				},
				0b00_00_10_10_10_01 | 0b00_10_00_10_10_01 | 0b01_10_00_10_10_00 | 0b01_10_10_10_00_00  => {//align3 close moins
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": Value += 100 align3_close");
						value += 100;
						j = 10;
				}
				0b10_01_01_01_01_00 | 0b00_01_01_01_01_10  => { //align4 close plus
					// dbg_line((line & 0b1111_1111_1111) as u16);
				// println!(": value -= 1000 align4_close");
					value -= 1000;
						j = 10;
				},
				0b01_10_10_10_10_00 | 0b00_10_10_10_10_01  => { //align4 close moins
					// dbg_line((line & 0b1111_1111_1111) as u16);
				// println!(": value += 1000 align4_close");
					value += 1000;
						j = 10;
				},
				align5_moins if (align5_moins & 0b11_11_11_11_11 == 0b10_10_10_10_10) => {
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": value += 10000000 align5");
					value += 10000000;
					j = 10;
				},
				align5_plus if (align5_plus & 0b11_11_11_11_11 == 0b01_01_01_01_01) => {
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": value -= 10000000 align5");
					value -= 10000000;
					j = 10;
				},
				align2plus if align2plus & 0b11_11_11_11 == 0b00_01_01_00 => { //+align2 open
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": value -= 100 align2 open");
					value -= 100;
						j = 6;
				},
				align2moins if align2moins & 0b11_11_11_11 == 0b00_10_10_00 => { // -align3 open
					// dbg_line((line & 0b1111_1111_1111) as u16);
					// println!(": Value += 100 align2 open");
						value += 100;
						j = 6;
				}
				_ => j = 2,
			}
			line>>=j;
		}
		// map_lines_values.insert(l, value);
		value
	}
// }

fn get_all_diag1(cells: &[u64; 19]) -> Vec<u64> {
	let mut vec: Vec<u64> = (4..SIZE).map(|x| down_diago!(cells, x, 0, x, 0)).collect();
	let vec2: Vec<u64> = (1..SIZE-4).map(|x| down_diago!(cells, 0, SIZE - 1 -x, x, SIZE - 1)).collect();
	vec.extend(vec2);
	vec
}

fn get_all_diag2(cells: &[u64; 19]) -> Vec<u64> {
	let mut vec: Vec<u64> = (0..SIZE-4).map(|x| up_diago!(cells, 0, SIZE - 1 - x, x, 0)).collect();
	let vec2: Vec<u64> = (1..SIZE-4).map(|y| up_diago!(cells, 0, SIZE -1, 0, y)).collect();
	vec.extend(vec2);
	vec
}

    pub fn eval(state: &Gameboard, stone: u8, depth: u8, map_board_values: &mut HashMap<[u64; SIZE], isize>) -> isize {

		if (state.black_captures >= 10 && stone == WHITE) || (state.white_captures >= 10 && stone == BLACK){
			-10000000 - (depth * 50) as isize
		} else if state.white_captures >= 10  && stone == WHITE || state.black_captures >= 10  && stone == BLACK {
			10000000 + (depth * 50) as isize
		} else if map_board_values.contains_key(&state.cells) {
			if stone == WHITE {
				*map_board_values.get(&state.cells).unwrap() + (state.white_captures * state.white_captures * 10) as isize + (depth * 50) as isize
			} else {
				- *map_board_values.get(&state.cells).unwrap() - (state.black_captures  * state.black_captures * 10) as isize - (depth * 50) as isize
			}
		} else {
			let mut all: Vec<u64> = (0..SIZE).map(|y| line_horizontal!(state.cells, 0, SIZE - 1, y as usize)).collect();
			let all_verti: Vec<u64> = (0..SIZE).map(|x| line_vertical!(state.cells[x as usize], 0 , SIZE -1)).collect();
			let all_diag_1 = get_all_diag1(&state.cells);
			let all_diag_2 = get_all_diag2(&state.cells);

			all.extend(all_verti);
			all.extend(all_diag_1);
			all.extend(all_diag_2);
			all.retain(|&elem| elem != 0);

			let value: isize = all.iter().map(|&e| evale_one_line(e)).sum();
			if stone == WHITE {
				// if value != 0 {
				// 	println!("EVAL: ");
				// 	printboard!(state.cells);
				// 	println!("BOARD VALUE: {} (WHITE)\n---------------\n", -value);
				// }
				map_board_values.insert(state.cells, value);
				value + (state.white_captures * state.white_captures * 10) as isize + (depth * 50) as isize
			} else {
				// if value != 0 {
				// 	println!("EVAL: ");
				// 	printboard!(state.cells);
				// 	println!("BOARDVALUE: {} (BLACK)\n---------------\n", value);
				// }
				map_board_values.insert(state.cells, value);

				-value - (state.black_captures  * state.black_captures * 10) as isize - (depth * 50) as isize
			}
		}
	}
