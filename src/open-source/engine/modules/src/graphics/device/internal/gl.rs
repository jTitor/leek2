/*!
	Implements OpenGL graphics operations.
*/
//Now that gfx-rs is using a generic
//API under gfx::hal, we don't have
//to use this gross super-specific type anymore
use gfx_hal::Adapter;
use gfx_backend_gl::{Surface, Backend};

use graphics;
use graphics::BackendType;

type GLAdapter = Adapter<Backend>;

pub struct GLDevice {
	/**
	The backend destination for rendering calls.
	*/
	_impl_surface: Surface,
	_impl_adapters: Vec<GLAdapter>
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

impl Drop for GLDevice {
	fn drop(&mut self) {
		//All gfx fields need to be manually dropped!
	}
}

pub struct GLBuilder {
}

impl GLBuilder {
	pub fn build(surface: Surface, adapters: Vec<GLAdapter>) -> Result<GLDevice, ()> {
		let result = GLDevice {
				_impl_surface: surface,
				_impl_adapters: adapters
			};

		//Those properties let us make the backend
		//device, generate that here.

		//Generate a default pipeline here:
		//	A default layout...
		//	With a single pass...
		//	And a single vertex and single fragment shader.

		//Generate descriptors for the sampler/sampled image
		//for the shaders to bind to.

		//Setup the framebuffer handles and render targets.
		//	Allocate space for the buffers.


		Ok(result)
	}
}