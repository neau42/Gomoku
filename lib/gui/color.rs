pub fn fill_color(red: u8, green: u8, blue: u8, alpha: f32) -> [f32; 4] {
	let red = f32::from(red as f32 / 255 as f32);
	let green = f32::from(green as f32 / 255 as f32);
	let blue = f32::from(blue as f32 / 255 as f32);
	[red, green, blue, alpha]
}