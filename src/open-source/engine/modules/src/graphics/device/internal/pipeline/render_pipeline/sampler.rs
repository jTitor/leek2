/*!
 * Samples color data from a texture.
 */
use graphics::device::internal::pipeline::{DeviceController, DeviceResource};

use gfx_hal as hal;

pub struct Sampler<B> where B: hal::Backend {
	unimplemented!()
	sampler: B::Sampler,
	/**
	 * The Sampler's id in the
	 * DeviceController's buffer list.
	 */
	sampler_device_id: usize,
	resources_destroyed: bool
}

impl<B> Sampler<B> where B: hal::Backend {
	pub fn build() -> Sampler<B> {
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

		unimplemented!()
	}

	pub fn mark_destroyed(&mut self) {
		debug_assert!(!self.resources_destroyed,
			"Sampler already marked as destroyed");

		self.resources_destroyed = true;
	}
}

impl<B> Drop for Sampler<B> where B: hal::Backend {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed, "MemoryBuffer went out of scope without having its memory destroyed");
	}
}

impl<B> DeviceResource<Sampler<B>> for DeviceController<B> where B: hal::Backend {
	fn get_resource(&mut self) -> Weak<&Sampler<B>> {
		unimplemented!()
	}

	fn destroy_all_resources<Sampler<B>>(&mut self) -> Result<(), Error> {
		// for sampler in self.resource_lists.samplers {
		// 	self.device.destroy_sampler(sampler);
		// }
		unimplemented!()

		Ok(())
	}

	fn destroy_resource(&mut self, resource: &mut T) -> Result<(), Error> {
		self.device.destroy_sampler(self.sampler);
		unimplemented!()

		resource.mark_destroyed();

		Ok(())
	}
}