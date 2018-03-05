/*!
 * Handles buffer storage for a graphics::Device.
 * This buffer is generated from data loaded from disk.
 */
use graphics::device::internal::pipeline::{DeviceController, DeviceResource, MemoryBuffer};

use std::rc::Weak;

use gfx_hal as hal;
use gfx_hal::Device;
use failure::Error;

pub struct WriteableBuffer<B: hal::Backend> {
	memory_buffer: MemoryBuffer<B>
}

impl<B: hal::Backend> WriteableBuffer<B> {
	fn build() -> WriteableBuffer<B> {
		unimplemented!()
	}

	fn write_to_buffer(&self, data: Vec<u8>, device: &mut B::Device) -> Result<(), Error> {
		let mut vertices = device
			.acquire_mapping_writer::<Vertex>(&self.memory_buffer.buffer_memory, 0..self.memory_buffer.buffer_len)?;
		vertices.copy_from_slice(data.as_slice());

		//Presumably this returns a Result as well.
		device.release_mapping_writer(vertices);

		Ok(())
	}
}

//TODO_rust: make this into a factory that outputs a specially-made MemoryBuffer instead?
//Alternately - have this wrap a MemoryBuffer and implement Into<MemoryBuffer>.

impl<B: hal::Backend> From<WriteableBuffer<B>> for MemoryBuffer<B> {
	fn from(x: WriteableBuffer<B>) -> MemoryBuffer<B> {
		x.memory_buffer
	}
}

pub trait WriteableBufferCapability {}
impl<B: hal::Backend> WriteableBufferCapability for WriteableBuffer<B> {}

impl<B: hal::Backend, C: WriteableBufferCapability> DeviceResource<C> for DeviceController<B> {
	fn get_resource<C>(&mut self) -> Weak<&C> {
		debug_assert!(false, "Can't directly get_resource() for WriteableBuffer; get_resource() on MemoryBufferBuilder instead and then call MemoryBuffer::into_writeable_buffer() instead");

		//Return a blank ref
		Weak::<&C>::new()
	}

	fn destroy_all_resources<C>(&mut self) -> Result<(), Error> {
		//TODO: Unwrap and destroy the
		//internal buffer
		unimplemented!()
	}

	fn destroy_resource<C>(&mut self, resource: &mut C) -> Result<(), Error> {
		//TODO: Unwrap and destroy the
		//internal buffer
		unimplemented!()
	}
}