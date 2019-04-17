use crate::models::gameboard::*;
use std::collections::HashMap;

const BLACK_WHITE: u16 =			0b00_00_00_01_10_00;
const WHITE_BLACK: u16 =			0b00_00_00_10_01_00;
const EMPTY: u16 =					0b00_00_00_00_00_00;
const ONE_BLACK: u16 =				0b00_00_00_00_01_00;
const ONE_WHITE: u16 =				0b00_00_00_00_10_00;
const FOUR_BLACK: u16 =				0b00_01_01_01_01_00;
const FOUR_WHITE: u16 =				0b00_10_10_10_10_00;

const THREE_BLACK_CLOSE1: u16 =		0b00_00_01_01_01_10;
const THREE_BLACK_CLOSE2: u16 =		0b10_01_00_01_01_00;
const THREE_BLACK_CLOSE3: u16 =		0b10_01_01_01_00_00;

const THREE_WHITE_CLOSE1: u16 =		0b00_00_10_10_10_01;
const THREE_WHITE_CLOSE2: u16 =		0b01_10_00_10_10_00;
const THREE_WHITE_CLOSE3: u16 =		0b01_10_10_10_00_00;

const FOUR_BLACK_CLOSE1: u16 =		0b10_01_00_01_01_01;
const FOUR_BLACK_CLOSE2: u16 =		0b10_01_01_01_00_01;
const FOUR_BLACK_CLOSE3: u16 =		0b10_01_01_01_01_00;
const FOUR_BLACK_CLOSE4: u16 =		0b00_01_01_01_01_10;
const FOUR_BLACK_CLOSE5: u16 =		0b01_00_01_01_01_00;
const FOUR_BLACK_CLOSE6: u16 =		0b01_01_00_01_01_00;
const FOUR_BLACK_CLOSE7: u16 =		0b00_01_01_01_00_01;

const FOUR_WHITE_CLOSE1: u16 =		0b01_10_00_10_10_10;
const FOUR_WHITE_CLOSE2: u16 =		0b01_10_10_10_00_10;
const FOUR_WHITE_CLOSE3: u16 =		0b01_10_10_10_10_00;
const FOUR_WHITE_CLOSE4: u16 =		0b00_10_10_10_10_01;
const FOUR_WHITE_CLOSE5: u16 =		0b10_00_10_10_10_00;
const FOUR_WHITE_CLOSE6: u16 =		0b10_10_00_10_10_00;
const FOUR_WHITE_CLOSE7: u16 =		0b00_10_10_10_00_10;

const TWO_BLACK_OPEN: u16 =			0b00_00_00_01_01_00;
const TWO_WHITE_OPEN: u16 =			0b00_00_00_10_10_00;

const TWO_BLACK_OPEN_HOLE: u16 =	0b00_00_01_00_01_00;
const TWO_WHITE_OPEN_HOLE: u16 =	0b00_00_10_00_10_00;

const THREE_BLACK_OPEN: u16 =		0b00_00_01_01_01_00;
const THREE_WHITE_OPEN: u16 =		0b00_00_10_10_10_00;

const THREE_BLACK_OPEN_HOLE1: u16 =	0b00_01_00_01_01_00;
const THREE_BLACK_OPEN_HOLE2: u16 =	0b00_01_01_00_01_00;

const THREE_WHITE_OPEN_HOLE1: u16 =	0b00_10_00_10_10_00;
const THREE_WHITE_OPEN_HOLE2: u16 =	0b00_10_10_00_10_00;

pub const BLACK_5_ALIGN: u16 =		0b00_01_01_01_01_01;
pub const WHITE_5_ALIGN: u16 =		0b00_10_10_10_10_10;

pub fn evale_one_line(l: u64) -> isize {
	let mut value = 0;
	let mut j: isize;
	let mut line = l;
	while line != 0 {
		match (line & 0b11_11_11_11_11_11) as u16 {
			EMPTY | ONE_BLACK | ONE_WHITE | BLACK_WHITE | WHITE_BLACK => {
				j = 10;
			},
			align5_white if (align5_white & 0b11_11_11_11_11 == WHITE_5_ALIGN) => {
				value += 10_000_000;
				j = 10;
			},
			align5_black if (align5_black & 0b11_11_11_11_11 == BLACK_5_ALIGN) => {
				value -= 10_000_000;
				j = 10;
			},
			FOUR_BLACK => {
				value -= 100_000;
				j = 10;
			},
			FOUR_WHITE => {
				value += 100_000;
				j = 10;
			},
			THREE_BLACK_CLOSE1 |
			THREE_BLACK_CLOSE2 |
			THREE_BLACK_CLOSE3 => {
				value -= 100;
				j = 10;
			},
			THREE_WHITE_CLOSE1 |
			THREE_WHITE_CLOSE2 |
			THREE_WHITE_CLOSE3 => {
				value += 100;
				j = 10;
			},
			FOUR_BLACK_CLOSE1 |
			FOUR_BLACK_CLOSE2 |
			FOUR_BLACK_CLOSE3 |
			FOUR_BLACK_CLOSE4 |
			FOUR_BLACK_CLOSE5 |
			FOUR_BLACK_CLOSE6 |
			FOUR_BLACK_CLOSE7
				=> {
				value -= 10_000;
				j = 8;
			},
			FOUR_WHITE_CLOSE1 |
			FOUR_WHITE_CLOSE2 |
			FOUR_WHITE_CLOSE3 |
			FOUR_WHITE_CLOSE4 |
			FOUR_WHITE_CLOSE5 |
			FOUR_WHITE_CLOSE6 |
			FOUR_WHITE_CLOSE7 => {
				value += 10_000;
				j = 8;
			},
			align2black_open if align2black_open & 0b00_11_11_11_11_11 == TWO_BLACK_OPEN => {
				value -= 100;
					j = 8;
			},
			align2white_open if align2white_open & 0b00_11_11_11_11_11 == TWO_WHITE_OPEN => {
				value += 100;
				j = 8;
			},
			align2black_hole if align2black_hole & 0b00_11_11_11_11_11 == TWO_BLACK_OPEN_HOLE => {
				value -= 10;
				j = 8;
			},
			align2white_hole if align2white_hole & 0b00_11_11_11_11_11 == TWO_WHITE_OPEN_HOLE => {
				value += 10;
				j = 8;
			},
			align3black if (align3black & 0b00_11_11_11_11_11) == THREE_BLACK_OPEN => {
				value -= 10000;
				j = 8;
			},
			align3black if (align3black & 0b00_11_11_11_11_11) == THREE_BLACK_OPEN_HOLE1
						|| (align3black & 0b00_11_11_11_11_11) == THREE_BLACK_OPEN_HOLE2 => {
				value -= 1000;
				j = 8;
			},
			align3white if (align3white & 0b00_11_11_11_11_11) == THREE_WHITE_OPEN => {
				value += 10000;
				j = 8;
			}
			align3white if (align3white & 0b00_11_11_11_11_11) == THREE_WHITE_OPEN_HOLE1
						|| (align3white & 0b00_11_11_11_11_11) == THREE_WHITE_OPEN_HOLE2 => {
				value += 1000;
				j = 8;
			}
			_ => j = 2,
		}
		line>>=j;
	}
	value
}

fn get_all_diag1(cells: &[u64; SIZE]) -> Vec<u64> {
	let mut vec: Vec<u64> = (4..SIZE).map(|x| down_diago!(cells, x, 0, x, 0)).collect();
	let vec2: Vec<u64> = (1..SIZE-4).map(|x| down_diago!(cells, 0, SIZE - 1 -x, x, SIZE - 1)).collect();
	vec.extend(vec2);
	vec
}

fn get_all_diag2(cells: &[u64; SIZE]) -> Vec<u64> {
	let mut vec: Vec<u64> = (0..SIZE-4).map(|x| up_diago!(cells, 0, SIZE - 1 - x, x, 0)).collect();
	let vec2: Vec<u64> = (1..SIZE-4).map(|y| up_diago!(cells, 0, SIZE -1 - y, 0, y)).collect();
	vec.extend(vec2);
	vec
}

pub fn eval(state: &Gameboard, actual_stone: u8, depth: u8, map_board_values: &mut HashMap<[u64; SIZE], isize>, player_stone: u8) -> isize {
	let mut score = if state.black_captures >= 10 {
		-10_000_000
	} else if state.white_captures >= 10 {
		10_000_000
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
	if actual_stone == player_stone {
		score
	} else {
		-score
	}
}
