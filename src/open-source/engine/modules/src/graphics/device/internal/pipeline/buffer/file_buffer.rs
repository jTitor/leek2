/*!
 * Handles buffer storage for a graphics::Device.
 * This buffer is generated from data loaded from disk.
 */
pub struct FileBuffer {
	unimplemented!()
}

impl FileBuffer {
	pub fn build() -> FileBuffer {

	}

	/**
	 * Copies the given data into the
	 * memory buffer.
	 * Returns a Result indicating if the
	 * copy succeeded or the reason why the
	 * copy failed.
	 */
	fn write_to_buffer(&self, data: ?) -> Result<(), Error> {
		
	}
}

//TODO_rust: make this into a factory that outputs a specially-made MemoryBuffer instead?
//Alternately - have this wrap a MemoryBuffer and implement Into<MemoryBuffer>.