/*!
	Implements a window via Glutin+Gfx.
*/
use glutin;

use graphics;
use graphics::{Window, Visibility};
use math::Vec2;

#[derive(Debug)]
pub struct GlutinWindow {
	/**
	The actual backend
	implementation that manages the window.
	*/
	impl_window: glutin::Window,
	visibility: Visibility
}

impl GlutinWindow {
	pub fn new(glutin_window: glutin::Window) -> GlutinWindow {
		GlutinWindow {
			impl_window: glutin_window,
			visibility: Visibility::Closed
		}
	}
}

impl graphics::Window for GlutinWindow {
	fn title(&self) -> &str {
		unimplemented!()
	}

	fn visibility(&self) -> Visibility {
		self.visibility
	}

	fn position(&self) -> Vec2 {
		if let Some(position) = self.impl_window.get_position() {
			return Vec2::new(position.0 as f32, position.1 as f32);
		}
		Vec2{}
	}

	fn dimensions(&self) -> Vec2 {
		if let Some(dimensions) = self.impl_window.get_inner_size_pixels() {
			return Vec2::new(dimensions.0 as f32, dimensions.1 as f32);
		}
		Vec2{}
	}

	fn is_open(&self) -> bool {
		return self.visibility != Visibility::Closed;
	}

	fn is_visible(&self) -> bool {
		match self.visibility {
			Visibility::Closed => return false,
			Visibility::Minimized => return false,
			_ => return true
		}
	}

	fn open(&self) {
		self.impl_window.show();
		//Not clear how we handle it;
		//show() returns void,
		//so how do we pick up a panic?
		unimplemented!()
	}

	fn hide(&self) {
		self.impl_window.hide();
		//See GlutinWindow.open() impl.
		unimplemented!()
	}

	fn swap_buffers(&self) -> Result<(), unimplemented!()> {
		let result = self.impl_window.swap_buffers();
		//You probably want to genericize this
		//error.
		unimplemented!()
		result
	}
}