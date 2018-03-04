/*!
 * Handles buffer storage for a graphics::Device.
 * This buffer is manually specified in size and format.
 */
use std::rc::Weak;

use failure::Error;
use gfx_hal as hal;
use hal::buffer;

//TODO: Make buffer a T
//so it operates on images and buffers
pub struct MemoryBuffer<B> where B: hal::Backend {
	device: Weak<&hal::Device<B>>,
	pub buffer: B::UnboundBuffer,
	pub buffer_memory: B::Memory,
	pub buffer_binding: B::Buffer,
	/** The MemoryBuffer's id in the
	 * DeviceController's buffer list
	 */
	buffer_device_id: usize,
	pub buffer_len: usize,
	unimplemented!()
	resources_destroyed: bool;
}

impl<B> MemoryBuffer<B> where B: hal::Backend {
	fn build() -> Result<MemoryBuffer<B>, Error> {
		// Buffer allocations
		println!("Memory types: {:?}", memory_types);

		let buffer_stride = std::mem::size_of::<Vertex>() as u64;
		let buffer_len = QUAD.len() as u64 * buffer_stride;

		let buffer_unbound = device.create_buffer(buffer_len, buffer::Usage::VERTEX)?;
		let buffer_req = device.get_buffer_requirements(&buffer_unbound);

		let upload_type = memory_types
			.iter()
			.enumerate()
			.position(|(id, mem_type)| {
				buffer_req.type_mask & (1 << id) != 0 &&
				mem_type.properties.contains(m::Properties::CPU_VISIBLE)
			})?
			.into();

		let buffer_memory = device.allocate_memory(upload_type, buffer_req.size)?;
		let vertex_buffer = device.bind_buffer_memory(&buffer_memory, 0, buffer_unbound)?;

		Ok(unimplemented!())
	}

	pub fn destroy_resources(&mut self) {
		debug_assert!(!self.resources_destroyed);

		if !self.resources_destroyed {
			self.device.destroy_buffer(self.buffer);
			self.device.free_memory(self.buffer_memory);

			self.resources_destroyed = true;
		}
	}
}

impl<B> Drop for MemoryBuffer<B> where B: hal::Backend {
	fn drop(&mut self) {
		if !self.resources_destroyed {
			self.destroy_resources();
		}
	}
}