/*!
	Implements OpenGL graphics operations.
*/
//Now that gfx-rs is using a generic
//API under gfx::hal, we don't have
//to use this gross super-specific type anymore
use gfx_hal;
use gfx_backend_gl;

use graphics;
use graphics::BackendType;

type GfxGlDevice = gfx_device_gl::Device;
type GfxGlDeviceFactory = gfx_device_gl::Factory;
type GfxRenderTargetView = gfx_core::handle::RenderTargetView<gfx_device_gl::Resources, gfx_core::format::Rgba8>;
type GfxDepthStencilView = gfx_core::handle::DepthStencilView<gfx_device_gl::Resources, gfx_core::format::DepthStencil>;

pub struct GLDevice {
	/**
	The backend destination for rendering calls.
	*/
	impl_surface: gfx_backend_gl::Surface,
	impl_adapters: Vec<gfx_hal::Adapter>
}

//TODO: impl Debug for GLDevice

impl graphics::Device for GLDevice {
	fn backend_type(&self) -> BackendType {
		BackendType::OpenGL
	}
	
	fn end_frame(&mut self) {
		//Meant to be empty
	}
}

impl GLDevice {
	pub fn new(surface: gfx_backend_gl::Surface, adapters: Vec<gfx_hal::Adapter>) -> GLDevice {
		GLDevice {
			impl_surface: surface,
			impl_adapters: adapters
		}
	}
}