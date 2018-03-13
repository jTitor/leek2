/*!
 * Defines the WritableBuffer struct.
 */
use graphics::device::internal::pipeline::{DeviceResource, MemoryBuffer};

use std::rc::Weak;
use std::marker::PhantomData;

use gfx_hal as hal;
use gfx_hal::Device;
use failure::Error;

/**
 * Wrapper around a MemoryBuffer that allows
 * writing data to a MemoryBuffer.
 */
pub struct WriteableBuffer<B: hal::Backend, ElementT: Copy> {
	memory_buffer: MemoryBuffer<B>,
	_element_type: PhantomData<ElementT>
}

impl<B: hal::Backend, ElementT: Copy> WriteableBuffer<B, ElementT> {
	fn build() -> WriteableBuffer<B, ElementT> {
		unimplemented!()
	}

	fn write_to_buffer(&self, data: &Vec<ElementT>, device: &mut B::Device) -> Result<(), Error> {
		let mut vertices = device
			.acquire_mapping_writer::<ElementT>(&self.memory_buffer.buffer_memory, 0..self.memory_buffer.buffer_len)?;
		vertices.copy_from_slice(data.as_slice());

		//Presumably this returns a Result as well.
		device.release_mapping_writer(vertices);

		Ok(())
	}
}

//TODO_rust: make this into a factory that outputs a specially-made MemoryBuffer instead?
//Alternately - have this wrap a MemoryBuffer and implement Into<MemoryBuffer>.

impl<B: hal::Backend, ElementT: Copy> From<WriteableBuffer<B, ElementT>> for MemoryBuffer<B> {
	fn from(x: WriteableBuffer<B, ElementT>) -> MemoryBuffer<B> {
		x.memory_buffer
	}
}

impl<B: hal::Backend, ElementT: Copy> DeviceResource<B> for WriteableBuffer<B, ElementT> {
	fn get_resource(device: &mut B::Device) -> Weak<&Self> {
		debug_assert!(false, "Can't directly get_resource() for WriteableBuffer; get_resource() on MemoryBufferBuilder instead and then call MemoryBuffer::into_writeable_buffer() instead");

		//Return a blank ref
		Weak::<&Self>::new()
	}
	fn destroy_resource(device: &mut B::Device, resource: &mut Self) -> Result<(), Error> {
		//TODO: Unwrap and destroy the
		//internal buffer
		unimplemented!()
	}

	fn resources_destroyed(&self) -> bool {
		self.resources_destroyed;
	}
}