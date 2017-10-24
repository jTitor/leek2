/*!
	Implements OpenGL graphics operations.
*/
use gfx_core;
use gfx_core::Device;
use gfx_device_gl;

use graphics;
use graphics::BackendType;

type GfxGlDevice = gfx_device_gl::Device;
type GfxGlDeviceFactory = gfx_device_gl::Factory;
type GfxRenderTargetView = gfx_core::handle::RenderTargetView<gfx_device_gl::Resources, gfx_core::format::Rgba8>;
type GfxDepthStencilView = gfx_core::handle::DepthStencilView<gfx_device_gl::Resources, gfx_core::format::DepthStencil>;

pub struct GLDevice {
	/**
	The actual backend
	implementation that runs the graphics calls.
	*/
	impl_device: GfxGlDevice,
	impl_factory: GfxGlDeviceFactory,
	impl_render_target_view: GfxRenderTargetView,
	impl_depth_stencil_view: GfxDepthStencilView
}

//TODO: impl Debug for GLDevice

impl graphics::Device for GLDevice {
	fn backend_type(&self) -> BackendType {
		BackendType::OpenGL
	}
	
	fn end_frame(&mut self) {
		self.impl_device.cleanup()
	}
}

impl GLDevice {
	pub fn new(device: GfxGlDevice, factory: GfxGlDeviceFactory, render_view: GfxRenderTargetView, depth_stencil_view: GfxDepthStencilView) -> GLDevice {
		GLDevice {
			impl_device: device,
			impl_factory: factory,
			impl_render_target_view: render_view,
			impl_depth_stencil_view: depth_stencil_view
		}
	}
}