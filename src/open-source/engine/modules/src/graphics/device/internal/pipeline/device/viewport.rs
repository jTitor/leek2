/*!
 * Specifies the geometry of a viewport's
 * rendering volume.
 */

pub struct Viewport {
	unimplemented!()
}

impl Default for Viewport {
	fn default() -> Self {
		let viewport = command::Viewport {
			rect: command::Rect {
				x: 0, y: 0,
				w: pixel_width, h: pixel_height,
			},
			depth: 0.0 .. 1.0,
		};
	}
}