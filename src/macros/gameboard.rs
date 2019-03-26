macro_rules! get_stone {
	($line: expr, $y: expr) => {
		($line >> ($y * 2) & 0b11) as u8
	};
}

macro_rules! clear_stone {
	($y: expr) => {
		 !(0b11 << ($y * 2) as u64)
	};
}

macro_rules! set_stone {
	($y: expr, $stone: expr) => {
		($stone as u64) << ($y * 2)
	};
}

macro_rules! set_move {
	($y: expr) => {
		0b1 << $y
	};
}

macro_rules! opposite_stone {
	($stone: expr) => {
		!$stone & 0b11
	};
}

macro_rules! line_horizontal {
	($cells: expr, $x_min: expr, $x_max: expr, $y: expr) => {
		($x_min..=$x_max).enumerate().fold(0, |value, (index, x)| {
			value | ((get_stone!($cells[x], $y) as u64) << (index * 2))
		})
	};
}

macro_rules! line_vertical {
	($line: expr, $y_min: expr, $y_max: expr) => {
		(($line >> ($y_min * 2)) as u64) & ((1 << ($y_max * 2 + 1)) - 1)
	};
}

macro_rules! up_diago {
	($cells: expr, $diago_up_left: expr, $diago_down_right: expr, $x_orig: expr, $y_orig: expr) => {
		(($x_orig - $diago_up_left)..=($x_orig + $diago_down_right))
		.enumerate()
		.fold(0, |value, (index, x)| {
			value | ((get_stone!($cells[x], $y_orig - $diago_up_left + index) as u64) << (index * 2))
		})
	};
}

macro_rules! down_diago {
	($cells: expr, $diago_down_left: expr, $diago_up_right: expr, $x_orig: expr, $y_orig: expr) => { 
		(($x_orig - $diago_down_left)..=($x_orig + $diago_up_right))
			.enumerate()
			.fold(0, |value , (index, x)| {
				value | ((get_stone!($cells[x], $y_orig + $diago_down_left - index) as u64) << (index * 2))
			})
	};
}

macro_rules! get_tree_forms {
	($stone: expr) => {
		match $stone {
			WHITE => WHITE_TREES,
			_ => BLACK_TREES,
		}
	}
}

macro_rules! tree_lines {
	($cells: expr, $x: expr, $x_min: expr, $x_max: expr, $y: expr, $y_min: expr, $y_max: expr, $diago_up_left: expr, $diago_down_right: expr, $diago_down_left: expr, $diago_up_right: expr) => {
		[
			line_horizontal!($cells, $x_min, $x_max, $y) as u32,
			line_vertical!($cells[$x], $y_min, $y_max) as u32,
			down_diago!($cells, $diago_down_left, $diago_up_right, $x, $y) as u32,
			up_diago!($cells, $diago_up_left, $diago_down_right, $x, $y) as u32,
		]
	};
}


macro_rules! get_capture_form {
	($stone: expr) => {
		match $stone {
			WHITE => WHITE_CAPTURE,
			_ => BLACK_CAPTURE,
		}
	}
}

/// NE PAS LIRE
macro_rules! capture_lines {
	($cells: expr, $x: expr, $x_min: expr, $x_max: expr, $y: expr, $y_min: expr, $y_max: expr, $diago_up_left: expr, $diago_down_right: expr, $diago_down_left: expr, $diago_up_right: expr) => {
		// {
			// println!("{}|{}|{}", line_vertical!($cells[$x], $y_min, $y), $y_min, $y);
		[
			((up_diago!($cells, $diago_up_left, 0, $x, $y) >> (2 - (5 - ($diago_up_left))) * 2) as u8, (-1, -1)),
			((line_horizontal!($cells, $x_min, $x, $y) >> (2 - (5 - ($x - $x_min))) * 2) as u8, (-1, 0)),
			((down_diago!($cells, $diago_down_left, 0, $x, $y) >> (2 - (5 - ($diago_down_left))) * 2) as u8,(-1, 1)),
			(down_diago!($cells, 0, $diago_up_right, $x, $y) as u8, (1, -1)),
			(line_horizontal!($cells, $x, $x_max, $y) as u8, (1, 0)),
			(up_diago!($cells, 0, $diago_down_right, $x, $y) as u8 , (1, 1)),
			((line_vertical!($cells[$x], $y_min, $y) >> (2 - (5 - ($y - $y_min))) * 2) as u8, (0, -1)),
			(line_vertical!($cells[$x], $y, $y_max) as u8, (0, 1)),
		]
		// }
	};
}

macro_rules! concat_stones {
	($line: expr, $nbr_stone: expr) => {
		($line & ((1 << $nbr_stone * 2) - 1))
	}
}

macro_rules! printboard {
	($cells: expr) => {
		print!("BOARD:\n   ");
		for x in 0..SIZE { print!("{0: <2} ", x) };
		println!();

		for y in 0..SIZE {
			print!("{0: <2} ", y);
			for x in 0..SIZE {
				match get_stone!($cells[x], y) {
					WHITE => print!("W  "),
					BLACK => print!("B  "),
					_ => print!(".  ")
				}
			}
			println!();
		}
	};
}


macro_rules! up_diago_orig {
	($len_origin_min: expr, $len_origin_max: expr, $cells: expr, $x_orig: expr, $x_min: expr, $x_max: expr, $y_orig: expr, $y_min: expr, $y_max: expr) => {
		(($x_orig - $len_origin_min)..=($x_orig + $len_origin_max))
		.enumerate()
		.fold(0, |value, (index, x)| {
			value | ((get_stone!($cells[x], $y_orig - $len_origin_min + index) as u64) << (index * 2))
		})
	};

	($cells: expr, $x_orig: expr, $x_min: expr, $x_max: expr, $y_orig: expr, $y_min: expr, $y_max: expr) => {
		up_diago_orig!(
			($y_orig - $y_min).min($x_orig - $x_min),
			($y_max - $y_orig).min($x_max - $x_orig),
			$cells, $x_orig, $x_min, $x_max, $y_orig, $y_min, $y_max)
	};
}

macro_rules! down_diago_orig {
	($len_origin_min: expr, $len_origin_max: expr, $cells: expr, $x_orig: expr, $x_min: expr, $x_max: expr, $y_orig: expr, $y_min: expr, $y_max: expr) => { 
		(($x_orig - $len_origin_min)..=($x_orig + $len_origin_max))
			.enumerate()
			.fold(0, |value , (index, x)| {
				value | ((get_stone!($cells[x], $y_orig + $len_origin_min - index) as u64) << (index * 2))
			})
	};

	($cells: expr, $x_orig: expr, $x_min: expr, $x_max: expr, $y_orig: expr, $y_min: expr, $y_max: expr) => {
		down_diago_orig!(
			($y_max - $y_orig).min($x_orig - $x_min),
			($y_orig - $y_min).min($x_max - $x_orig),
			$cells, $x_orig, $x_min, $x_max, $y_orig, $y_min, $y_max)
	};
}

