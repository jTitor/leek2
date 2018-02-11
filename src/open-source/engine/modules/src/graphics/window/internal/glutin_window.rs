/*!
	Implements a window via Glutin+Gfx.
*/
use glutin;
use glutin::GlContext;
use winit::{Window, EventsLoop};

use graphics;
use graphics::{Visibility, WindowError, EventType};
use input::Controller;
use graphics::window::convert_winit_event;
use math::Vec2;

pub struct GlutinWindow {
	impl_events_loop: EventsLoop,
	/**
	The actual backend
	implementation that manages the window.
	*/
	impl_window: glutin::GlWindow,
	visibility: Visibility
}

//TODO: impl Debug for GlutinWindow

impl GlutinWindow {
	pub fn new(glutin_window: glutin::GlWindow, events_loop: EventsLoop) -> GlutinWindow {
		GlutinWindow {
			impl_events_loop: events_loop,
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
		Vec2::default()
	}

	fn dimensions(&self) -> Vec2 {
		if let Some(dimensions) = self.impl_window.get_inner_size_pixels() {
			return Vec2::new(dimensions.0 as f32, dimensions.1 as f32);
		}
		Vec2::default()
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
	}

	fn hide(&self) {
		self.impl_window.hide();
	}

	fn swap_buffers(&self) -> Result<(), WindowError> {
		match self.impl_window.swap_buffers() {
			Ok(_) => Ok(()),
			_ => Err(WindowError::BackendOperationFailed)
		}
	}

	fn poll_events(&mut self, callback: &mut FnMut(EventType)) {
		self.impl_events_loop.poll_events(|event| {
			//Wrap the glutin event in the engine event type
			let converted_event = convert_winit_event(event);
			//And run the callback on it
			callback(converted_event);
		});
	}

	fn get_input_devices(&self) -> Vec<Controller> {
		unimplemented!()
	}
}