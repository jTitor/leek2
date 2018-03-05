/*!
 * Specifies the geometry of a viewport's
 * rendering volume.
 */
use math::screen::Size;
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
				x: self.x as u16, y: self.y as u16,
				w: self.dimensions.width() as u16,
				h: self.dimensions.height() as u16,
			},
			depth: self.depth_near .. self.depth_far,
		}
	}
}