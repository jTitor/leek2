/*!
 * Defines the MemoryBuffer struct.
 */
use std::rc::Weak;

use graphics::device::internal::pipeline::DeviceResource;

use failure::Error;
use gfx_hal as hal;
use gfx_hal::Device;

/**
 * Handles buffer storage for a graphics::Device.
 * This buffer is manually specified in size and format.
 */
pub struct MemoryBuffer<B: hal::Backend> {
	//TODO: Make buffer a T
	//so it operates on images and buffers
	pub buffer: B::UnboundBuffer,
	pub buffer_memory: B::Memory,
	pub buffer_binding: B::Buffer,
	/**
	 * The MemoryBuffer's id in the
	 * DeviceController's buffer list.
	 */
	buffer_device_id: u64,
	pub buffer_len: u64,
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

impl<B: hal::Backend> DeviceResource<B> for MemoryBuffer<B> {
	fn get_resource(device: &mut B::Device) -> Weak<&Self> {
		debug_assert!(false, "Can't directly get_resource() for MemoryBuffer; get_resource() on MemoryBufferBuilder instead and then call MemoryBufferBuilder::build()");

		//Return a blank ref
		Weak::<&Self>::new()
	}

	fn destroy_all_resources(device: &mut B::Device, resource_list: &Vec<Self>) -> Result<(), Error> {
		// for buffer in self.resource_lists.buffers {
		// 	self.device.destroy_buffer(buffer);
		// }

		unimplemented!()
	}

	fn destroy_resource(device: &mut B::Device, resource: &mut Self) -> Result<(), Error> {
		device.destroy_buffer(resource.buffer_binding);
		device.free_memory(resource.buffer_memory);
		unimplemented!();

		resource.mark_destroyed();
	}
}