/*!
	Implements OpenGL graphics operations.
*/
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;

use super::super::super::device::{BackendType, Device};

pub struct GLDevice {
	/**
	The actual backend
	implementation that runs the graphics calls.
	*/
	impl_device: gfx_device_gl::Device
}

impl Device for GLDevice {
	pub fn backend_type(&self) -> BackendType {
		BackendType::OpenGL
	}
	
	pub fn end_frame(&mut self) {
		self.impl_device.cleanup()
	}
}

impl GLDevice {
	pub fn new(device: gfx_device_gl::Device) -> GLDevice {
		GLDevice {
			impl_device: device
		}
	}
}