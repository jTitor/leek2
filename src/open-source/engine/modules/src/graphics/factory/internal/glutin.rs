/*!
Builds a Device and Window via Gfx and Glutin.
*/
use std::sync::Arc;
use std::rc::Rc;

use gfx_hal::Instance;
use gfx_hal::format::{Rgba8Srgb, AsFormat};
use gfx_backend_gl;
use glutin;
use winit;

use graphics::GraphicsPayload;
use super::FactoryDispatcher;
use graphics::window::internal::glutin_window::GlutinWindow;
use graphics::device::internal::gl::GLDevice;
use math::Vec2Access;

#[derive(Debug)]
pub struct GlutinDeviceWindowBuilder {
}

impl GlutinDeviceWindowBuilder {
	pub fn new() -> GlutinDeviceWindowBuilder {
		GlutinDeviceWindowBuilder {}
	}

	pub fn build(&self, dispatcher: &FactoryDispatcher) -> GraphicsPayload {
		let factory = dispatcher.factory.clone();
		let window_request = factory.window_request.clone();
		let device_request = factory.device_request;

		//The window builder just specifies the
		//window's physical properties.
		let window_builder = gfx_backend_gl::glutin::WindowBuilder::new()
		.with_title(window_request.title.to_string())
		.with_dimensions(window_request.dimensions.x() as u32, window_request.dimensions.y() as u32);

		//The event loop is the access point into
		//the window's events, such as when it gets
		//a keypress or the window is resized.
		let mut event_loop = gfx_backend_gl::glutin::EventsLoop::new();

		//Now build the actual window.
		let window = {
			//The OpenGL context never gets directly accessed,
			//so we can keep it in this block.
			let context_builder = gfx_backend_gl::config_context(
				gfx_backend_gl::glutin::ContextBuilder::new(),
				Rgba8Srgb::SELF,
				None,).with_vsync(true);

			gfx_backend_gl::glutin::GlWindow::new(window_builder, context_builder, &event_loop).unwrap()
		};

		let surface = gfx_backend_gl::Surface::from_window(window);
		let adapters = surface.enumerate_adapters();

		//These are the result stucts to be packed
		//in the return payload.
		let result_window = Box::new(GlutinWindow::new(event_loop));
		let result_device = Box::new(GLDevice::new(surface, adapters));

		GraphicsPayload {
			device: result_device,
			window: result_window
		}
	}
}