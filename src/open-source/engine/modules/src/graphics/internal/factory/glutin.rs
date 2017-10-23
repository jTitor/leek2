/*!
Builds a Device and Window via Gfx and Glutin.
*/
use gfx;
use glutin;
use gfx_window_glutin;

use graphics::GraphicsPayload;
use super::FactoryDispatcher;
use super::super::window::GlutinWindow;
use super::super::device::gl::GLDevice;
use math::vec_base::Vec2Access;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

/*
An example of a Glutin event loop.

pub fn main() {
	let builder = glutin::WindowBuilder::new()
		.with_title("Triangle example".to_string())
		.with_dimensions(1024, 768)
		.with_vsync();
	let (window, mut device, mut factory, main_color, mut main_depth) =
		gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
	'main: loop {
		for event in window.poll_events() {
			match event {
				glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
				glutin::Event::Closed => break 'main,
				_ => {}
			}
		}
		window.swap_buffers().unwrap();
		device.cleanup();
	}
}
*/

#[derive(Debug)]
pub struct GlutinDeviceWindowBuilder {
}

impl GlutinDeviceWindowBuilder {
	pub fn new() -> GlutinDeviceWindowBuilder {
		GlutinDeviceWindowBuilder {}
	}

	pub fn build(&self, dispatcher: &FactoryDispatcher) -> GraphicsPayload {
		let factory = dispatcher.factory;
		let window_request = factory.window_request;
		let device_request = factory.device_request;
		let builder = glutin::WindowBuilder::new()
		.with_title(window_request.title.to_string())
		.with_dimensions(window_request.dimensions.x() as u32, window_request.dimensions.y() as u32);
		if window_request.vsync {
			builder = builder.with_vsync();
		}

		let (window, mut device, mut factory, main_color, mut main_depth) =
			gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, unimplemented!());

		let result_window = Box::new(GlutinWindow::new(window));
		//Isn't device a reference?
		let result_device = Box::new(GLDevice::new(device, factory, main_color, main_depth));

		GraphicsPayload {
			device: result_device,
			window: result_window
		}
	}
}