/*!
 * Handles buffer storage for a graphics::Device.
 * This buffer is manually specified in size and format.
 */
use std::rc::Weak;

use graphics::device::internal::pipeline::{DeviceController, DeviceResource};

use failure::Error;
use gfx_hal as hal;
use gfx_hal::Device;

//TODO: Make buffer a T
//so it operates on images and buffers
pub struct MemoryBuffer<B: hal::Backend> {
	pub buffer: B::UnboundBuffer,
	pub buffer_memory: B::Memory,
	pub buffer_binding: B::Buffer,
	/**
	 * The MemoryBuffer's id in the
	 * DeviceController's buffer list.
	 */
	buffer_device_id: usize,
	pub buffer_len: usize,
	resources_destroyed: bool
}

impl<B: hal::Backend> MemoryBuffer<B> {
	fn mark_destroyed(&mut self) {
		debug_assert!(!self.resources_destroyed, "MemoryBuffer already marked as destroyed");
		
		self.resources_destroyed = true;
	}
}

impl<B: hal::Backend> Drop for MemoryBuffer<B> {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed, "MemoryBuffer went out of scope without having its memory destroyed");
	}
}

pub trait MemoryBufferCapability {}
impl<B: hal::Backend> MemoryBufferCapability for MemoryBuffer<B> {}

impl<B: hal::Backend, C: MemoryBufferCapability> DeviceResource<C> for DeviceController<B> {
	fn get_resource<C>(&mut self) -> Weak<&C> {
		debug_assert!(false, "Can't directly get_resource() for MemoryBuffer; get_resource() on MemoryBufferBuilder instead and then call MemoryBufferBuilder::build()");

		//Return a blank ref
		Weak::<&C>::new()
	}

	fn destroy_all_resources<C>(&mut self) -> Result<(), Error> {
		// for buffer in self.resource_lists.buffers {
		// 	self.device.destroy_buffer(buffer);
		// }

		unimplemented!()
	}

	fn destroy_resource<C>(&mut self, resource: &mut C) -> Result<(), Error> {
		self.device.destroy_buffer(resource.buffer);
		self.device.free_memory(resource.buffer_memory);
		unimplemented!();

		resource.mark_destroyed();
	}
}