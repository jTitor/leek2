/*!
 * Specifies the geometry of a viewport's
 * rendering volume.
 */
use std::num;
use gfx_hal::command;

pub struct Viewport {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
	pub depth_near: f32,
	pub depth_far: f32
}

impl Default for Viewport {
	fn default() -> Self {
		Viewport {
			x: 0f32,
			y: 0f32,
			width: 1f32,
			height: 1f32,
			depth_near: 0f32,
			depth_far: 0f32
		}
	}
}

impl Into<command::Viewport> for Viewport {
	fn into(self) -> command::Viewport {
		command::Viewport {
			rect: command::Rect {
				x: self.x, y: self.y,
				w: max(1f32, self.width),
				h: max(1f32, self.height),
			},
			depth: self.depth_near .. self.depth_far,
		}
	}
}