/*!
 * Describes a 2D length and width.
 */
use math::{Vec2, Vec2Access};

#[derive(Default)]
pub struct Size {
	dimensions: Vec2
}

const MIN_VALUE: f32 = 1f32;

impl Size {
	pub fn width(&self) -> f32 { self.dimensions.x() }
	pub fn height(&self) -> f32 { self.dimensions.y() }
	pub fn set_width(&mut self, value: f32) {
		value = MIN_VALUE.max(value);
		*self.dimensions.mut_x() = value;
	}
	pub fn set_height(&mut self, value: f32) {
		value = MIN_VALUE.max(value);
		*self.dimensions.mut_y() = value;
	}

	pub fn new(width: f32, height: f32) -> Size {
		Size { dimensions: Vec2::new(width, height) }
	}
}

impl From<Vec2> for Size {
	fn from(x: Vec2) -> Size {
		Size { dimensions: x }
	}
}