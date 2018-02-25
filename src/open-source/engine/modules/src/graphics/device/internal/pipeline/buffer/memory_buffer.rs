/*!
 * Handles buffer storage for a graphics::Device.
 * This buffer is manually specified in size and format.
 */
use failure::Error;

pub struct MemoryBuffer {
	buffer: ?,
	buffer_memory: ?,
	unimplemented!()
	resources_destroyed: bool;
}

impl MemoryBuffer {
	fn build() -> MemoryBuffer {
		// Buffer allocations
		println!("Memory types: {:?}", memory_types);

		let buffer_stride = std::mem::size_of::<Vertex>() as u64;
		let buffer_len = QUAD.len() as u64 * buffer_stride;

		let buffer_unbound = device.create_buffer(buffer_len, buffer::Usage::VERTEX).unwrap();
		let buffer_req = device.get_buffer_requirements(&buffer_unbound);

		let upload_type = memory_types
			.iter()
			.enumerate()
			.position(|(id, mem_type)| {
				buffer_req.type_mask & (1 << id) != 0 &&
				mem_type.properties.contains(m::Properties::CPU_VISIBLE)
			})
			.unwrap()
			.into();

		let buffer_memory = device.allocate_memory(upload_type, buffer_req.size).unwrap();
		let vertex_buffer = device.bind_buffer_memory(&buffer_memory, 0, buffer_unbound).unwrap();
	}

	/**
	 * Copies the given data into the
	 * memory buffer.
	 * Returns a Result indicating if the
	 * copy succeeded or the reason why the
	 * copy failed.
	 */
	fn write_to_buffer(&self, data: ?) -> Result<(), Error> {
		let mut vertices = device
			.acquire_mapping_writer::<Vertex>(&buffer_memory, 0..buffer_len)?
		vertices.copy_from_slice(data);

		//Presumably this returns a Result as well.
		device.release_mapping_writer(vertices)
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

impl Drop for Sampler {
	fn drop(&mut self) {
		if !self.resources_destroyed {
			self.destroy_resources();
		}
	}
}