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
	fn width(&self) -> f32 { self.dimensions.x() }
	fn height(&self) -> f32 { self.dimensions.y() }
	fn set_width(&mut self, value: f32) {
		value = MIN_VALUE.max(value);
		*self.dimensions.mut_x() = value;
	}
	fn set_height(&mut self, value: f32) {
		value = MIN_VALUE.max(value);
		*self.dimensions.mut_y() = value;
	}
}