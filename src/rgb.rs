#[derive (Default, Clone)]
pub struct RGB {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl RGB {
	pub fn new(r: u8, g: u8, b: u8) -> Self {
		Self {
			r,g,b
		}
	}

	pub fn splat(val: u8) -> Self {
		Self::new(val, val, val)
	}
}