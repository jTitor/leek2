/*!
	Implements OpenGL graphics operations.
*/
//Now that gfx-rs is using a generic
//API under gfx::hal, we don't have
//to use this gross super-specific type anymore
use gfx_hal;
use gfx_hal::{Adapter};
use gfx_backend_gl::{Surface, Backend};

use graphics;
use graphics::BackendType;

pub struct GLDevice {
	/**
	The backend destination for rendering calls.
	*/
	impl_surface: Surface,
	impl_adapters: Vec<Adapter<Backend>>
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
	pub fn new(surface: Surface, adapters: Vec<Adapter<Backend>>) -> GLDevice {
		GLDevice {
			impl_surface: surface,
			impl_adapters: adapters
		}
	}
}