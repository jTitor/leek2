/*!
 * Defines the DescriptorPool struct.
 */
use graphics::device::internal::pipeline::device::DeviceResource;

use std::rc::Weak;

use failure::Error;
use gfx_hal::{ Device, self as hal, pso };

/**
 * Contains the actual data of descriptors used by
 * this pipeline. Can be reset() to invalidate all
 * descriptor sets generated from it; it has
 * to be destroyed by the device to free
 * the underlying resources it uses, however.
 */
pub struct DescriptorPool<B: hal::Backend> {
	pub pool_impl: B::DescriptorPool,
	resources_destroyed_val: bool
}

impl<B: hal::Backend> DescriptorPool<B> {
	fn new(device: &B::Device, num_sets: u32, set_info: &[pso::DescriptorRangeDesc]) -> Self {
		Self {
			pool_impl: device.create_descriptor_pool(num_sets, set_info),
			resources_destroyed_val: false
		}
	}
}

impl<B: hal::Backend> DeviceResource<B> for DescriptorPool<B> {
	fn get_resource(device: &B::Device) -> Weak<&Self> {
		unimplemented!();
	}

	fn destroy_resource(&mut self, device: &B::Device) -> Result<(), Error> {
		device.destroy_descriptor_pool(self.pool_impl);
		self.resources_destroyed_val = true;

		Ok(())
	}

	fn resources_destroyed(&self) -> bool {
		self.resources_destroyed_val
	}
}