/*!
 * Handles buffer storage for a graphics::Device.
 * This buffer is manually specified in size and format.
 */
use graphics::device::internal::pipeline::DeviceController;

use failure::Error;
use gfx_hal as hal;
use hal::buffer;

//TODO: Make buffer a T
//so it operates on images and buffers
pub struct MemoryBuffer<B> where B: hal::Backend {
	pub buffer: B::UnboundBuffer,
	pub buffer_memory: B::Memory,
	pub buffer_binding: B::Buffer,
	/**
	 * The MemoryBuffer's id in the
	 * DeviceController's buffer list.
	 */
	buffer_device_id: usize,
	pub buffer_len: usize,
	unimplemented!()
	resources_destroyed: bool;
}

impl<B> MemoryBuffer<B> where B: hal::Backend {
	pub fn mark_destroyed(&mut self) {
		debug_assert!(!self.resources_destroyed, "MemoryBuffer already marked as destroyed");
		
		self.resources_destroyed = true;
	}
}

impl<B> Drop for MemoryBuffer<B> where B: hal::Backend {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed, "MemoryBuffer went out of scope without having its memory destroyed");
	}
}

impl<B> DeviceResource<MemoryBuffer<B>> for DeviceController<B> where B: hal::Backend {
	fn get_resource(&mut self) -> Weak<&MemoryBuffer<B>> {
		debug_assert!(false, "Can't directly get_resource() for MemoryBuffer; get_resource() on MemoryBufferBuilder instead and then call MemoryBufferBuilder::build()");

		//Return a blank ref
		Weak<MemoryBuffer<B>>::new()
	}

	fn destroy_all_resources<MemoryBuffer<B>>(&mut self) -> Result<(), Error> {
		// for buffer in self.resource_lists.buffers {
		// 	self.device.destroy_buffer(buffer);
		// }

		unimplemented!()
	}

	fn destroy_resource(&mut self, resource: &mut T) -> Result<(), Error> {
		self.device.destroy_buffer(resource.buffer);
		self.device.free_memory(resource.buffer_memory);
		unimplemented!()

		resource.mark_destroyed();
	}
}