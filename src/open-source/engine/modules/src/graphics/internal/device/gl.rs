/*!
	Implements OpenGL graphics operations.
*/
use gfx_core;
use gfx_device_gl;

use graphics::{BackendType, Device};

pub struct GLDevice {
	/**
	The actual backend
	implementation that runs the graphics calls.
	*/
	impl_device: gfx_device_gl::Device,
	impl_factory: gfx_device_gl::Factory,
	impl_render_target_view: Box<gfx_core::handle::RenderTargetView>,
	impl_depth_stencil_view: gfx_core::handle::DepthStencilView
}

impl Device for GLDevice {
	fn backend_type(&self) -> BackendType {
		BackendType::OpenGL
	}
	
	fn end_frame(&mut self) {
		self.impl_device.cleanup()
	}
}

impl GLDevice {
	pub fn new(device: gfx_device_gl::Device, factory: gfx_device_gl::Factory, render_view: gfx_core::handle::RenderTargetView, depth_stencil_view: gfx_core::handle::DepthStencilView) -> GLDevice {
		GLDevice {
			impl_device: device,
			impl_factory: factory,
			impl_render_target_view: render_view,
			impl_depth_stencil_view: depth_stencil_view
		}
	}
}