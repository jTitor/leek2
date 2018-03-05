/*!
 * Builds a buffer for a list of elements,
 * such as a vertex list.
 */
use super::{BufferBuilder, MemoryBuffer};

use gfx_hal as hal;
use failure::Error;

pub struct ElementBufferBuilder<Element, B: hal::Backend> {
	buffer_builder: BufferBuilder<B>,
	num_elements: usize
}

impl<Element, B: hal::Backend> ElementBufferBuilder<Element, B> {
	pub fn buffer_builder(&mut self) -> &mut BufferBuilder<B> { self.buffer_builder }

	pub fn build(&self) -> Result<MemoryBuffer<B>, Error> {
		//TODO: use size_of<Element>() to get
		//the buffer's final size.
		unimplemented!();

		Ok(unimplemented!())
	}
}