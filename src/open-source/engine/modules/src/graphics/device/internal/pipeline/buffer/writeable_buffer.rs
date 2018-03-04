/*!
 * Handles buffer storage for a graphics::Device.
 * This buffer is generated from data loaded from disk.
 */
use graphics::device::internal::pipeline::{DeviceController, MemoryBuffer};
use gfx_hal as hal;

pub struct WriteableBuffer<B> where B: hal::Backend {
	memory_buffer: MemoryBuffer<B>
}

impl<B> WriteableBuffer<B> where B: hal::Backend {
	pub fn build() -> WriteableBuffer<B> {
		unimplemented!()
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
			.acquire_mapping_writer::<Vertex>(&self.memory_buffer.buffer_memory, 0..self.memory_buffer.buffer_len)?
		vertices.copy_from_slice(data);

		//Presumably this returns a Result as well.
		device.release_mapping_writer(vertices)
	}
}

//TODO_rust: make this into a factory that outputs a specially-made MemoryBuffer instead?
//Alternately - have this wrap a MemoryBuffer and implement Into<MemoryBuffer>.

impl<B> From<WriteableBuffer> for MemoryBuffer<B> where B: hal::Backend {
	fn From(x: WriteableBuffer<B>) -> MemoryBuffer<B> {
		x.memory_buffer
	}
}

impl<B> DeviceResource<WriteableBuffer<B>> for DeviceController<B> where B: hal::Backend {
	fn get_resource(&mut self) -> Weak<&WriteableBuffer<B>> {
		debug_assert!(false, "Can't directly get_resource() for WriteableBuffer; get_resource() on MemoryBufferBuilder instead and then call MemoryBuffer::into_writeable_buffer() instead");

		//Return a blank ref
		Weak<WriteableBuffer<B>>::new()
	}

	fn destroy_all_resources(&mut self) -> Result<(), Error> {
		//TODO: Unwrap and destroy the
		//internal buffer
		unimplemented!()
	}

	fn destroy_resource(&mut self, resource: &mut T) -> Result<(), Error> {
		//TODO: Unwrap and destroy the
		//internal buffer
		unimplemented!()
	}
}