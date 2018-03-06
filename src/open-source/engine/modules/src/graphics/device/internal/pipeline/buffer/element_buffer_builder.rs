/*!
 * Defines the ElementBufferBuilder struct.
 */
use super::{BufferBuilder, MemoryBuffer};

use std::marker::PhantomData;

use gfx_hal as hal;
use failure::Error;

/**
 * BufferBuilder wrapper that 
 * builds a buffer for a list of elements,
 * such as a vertex list.
 */
pub struct ElementBufferBuilder<B: hal::Backend, ElementT> {
	buffer_builder: BufferBuilder<B>,
	_element_type: PhantomData<ElementT>,
	_backend_type: PhantomData<B>,
	num_elements: usize,
}

impl<B: hal::Backend, ElementT> ElementBufferBuilder<B, ElementT> {
	pub fn buffer_builder(&mut self) -> &mut BufferBuilder<B> { &mut self.buffer_builder }

	pub fn build(&self) -> Result<MemoryBuffer<B>, Error> {
		//TODO: use size_of<Element>() to get
		//the buffer's final size.
		unimplemented!();

		Ok(unimplemented!())
	}
}