/*!
 * Specifies the geometry of a viewport's
 * rendering volume.
 */
use std::num;

use math::Size;
use gfx_hal::command;

#[derive(Default)]
pub struct Viewport {
	pub x: f32,
	pub y: f32,
	pub dimensions: Size,
	pub depth_near: f32,
	pub depth_far: f32
}

impl Into<command::Viewport> for Viewport {
	fn into(self) -> command::Viewport {
		command::Viewport {
			rect: command::Rect {
				x: self.x, y: self.y,
				w: self.dimensions.width(),
				h: self.dimensions.height(),
			},
			depth: self.depth_near .. self.depth_far,
		}
	}
}