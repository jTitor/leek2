/*!
 * Samples color data from a texture.
 */
use graphics::device::internal::pipeline::DeviceResource;

use std::rc::Weak;

use gfx_hal as hal;
use gfx_hal::Device;
use gfx_hal::image as i;
use failure::Error;

pub struct Sampler<B: hal::Backend> {
	sampler: B::Sampler,
	/**
	 * The Sampler's id in the
	 * DeviceController's buffer list.
	 */
	sampler_device_id: usize,
	resources_destroyed: bool
}

impl<B: hal::Backend> Sampler<B> {
	pub fn build(device: &mut B::Device) -> Sampler<B> {
		let sampler = device.create_sampler(
			i::SamplerInfo::new(
				i::FilterMethod::Bilinear,
				i::WrapMode::Clamp,
			)
		);

		//This *doesn't* need a binding
		//like a buffer would,
		//you can just pass it to a
		//description set.

		unimplemented!();
	}

	pub fn mark_destroyed(&mut self) {
		debug_assert!(!self.resources_destroyed,
			"Sampler already marked as destroyed");

		self.resources_destroyed = true;
	}
}

impl<B: hal::Backend> Drop for Sampler<B> {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed, "MemoryBuffer went out of scope without having its memory destroyed");
	}
}

impl<B: hal::Backend> DeviceResource<B> for Sampler<B> {
	fn get_resource(device: &mut B::Device) -> Weak<&Self> {
		unimplemented!()
	}

	fn destroy_all_resources(device: &mut B::Device, resource_list: &Vec<Self>) -> Result<(), Error> {
		// for sampler in self.resource_lists.samplers {
		// 	self.device.destroy_sampler(sampler);
		// }
		unimplemented!();

		Ok(())
	}

	fn destroy_resource(device: &mut B::Device, resource: &mut Self) -> Result<(), Error> {
		device.destroy_sampler(resource.sampler);
		unimplemented!();

		resource.mark_destroyed();

		Ok(())
	}
}