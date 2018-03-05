/*!
 * Builds a buffer for a list of elements,
 * such as a vertex list.
 */
use super::BufferBuilder;

use failure::Error;

pub struct ElementBufferBuilder<Element> {
	buffer_builder: BufferBuilder,
	num_elements: usize
	unimplemented!()
}

impl<Element> ElementBufferBuilder<Element> {
	pub fn buffer_builder(&mut self) -> &mut BufferBuilder { self.buffer_builder }

	pub fn build(&self) -> Result<MemoryBuffer, Error> {
		//TODO: use size_of<Element>() to get
		//the buffer's final size.
		unimplemented!()
	}
}