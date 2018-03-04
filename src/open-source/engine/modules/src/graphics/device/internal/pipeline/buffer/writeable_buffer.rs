/*!
 * Handles buffer storage for a graphics::Device.
 * This buffer is generated from data loaded from disk.
 */
use super::MemoryBuffer;
use gfx_hal as hal;

pub struct FileBuffer<B> where B: hal::Backend {
	memory_buffer: MemoryBuffer<B>
}

impl<B> FileBuffer<B> where B: hal::Backend {
	pub fn build() -> FileBuffer<B> {
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