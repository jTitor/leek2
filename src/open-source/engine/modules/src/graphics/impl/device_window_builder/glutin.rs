/*!
Builds a Device and Window via Gfx and Glutin.
*/
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::Device;

use graphics::WindowBuilder;
use super::interface::DeviceWindowBuilderPayload;
use super::super::window::glutin::GlutinWindow;
use super::super::device::gl::GLDevice;

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
pub structGlutinDeviceWindowBuilder {
	//Make this into an info payload that
	//both builders use?
	base_info: &WindowBuilder
}

impl GlutinDeviceWindowBuilder {
	pub fn build(&self) -> DeviceWindowBuilderPayload {
		let builder = glutin::WindowBuilder::new()
		.with_title(self.base_info.title.to_string())
		.with_dimensions(self.base_info.dimensions.x(), self.base_info.dimensions.y())
		.with_vsync(self.base_info.vsync);

		let (window, mut device, mut factory, main_color, mut main_depth) =
			gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

		let result_window = GlutinWindow::new(window);
		//Isn't device a reference?
		let result_device = GLDevice::new(device);

		//TODO: What do we do with the factory,
		//main_color and main_depth?
		BuilderPayload {
			window: result_window,
			device: result_device
		}
	}
}