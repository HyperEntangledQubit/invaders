use crate::frame::Drawable;
use crate::frame::Frame;

const MAX_LEVEL: u8 = 3;

pub struct Level {
	level: u8,
}

impl Level {
	pub fn new() -> Self {
		Self {level: 1}
	}
	pub fn increment_level(&mut self) -> bool {
		if self.level <= MAX_LEVEL {
			self.level += 1;
		}
		self.level == MAX_LEVEL
	}
}

impl Default for Level {
	fn default() -> Self {
		Self::new()
	}
}

impl Drawable for Level {
	fn draw(&self, frame: &mut Frame) {
		let formatted = format!("LEVEL: {:0>2}", self.level);

		for (i, c) in formatted.chars().enumerate() {
			frame[i+20][0] = c;
		}
	}
}
