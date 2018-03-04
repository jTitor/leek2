/*!
 * Samples color data from a texture.
 */
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

	pub fn destroy_resources(&mut self) {
		debug_assert!(!self.resources_destroyed);

		if !self.resources_destroyed {
			self.device.destroy_sampler(self.sampler);

			self.resources_destroyed = true;
		}
	}
}

impl<B> Drop for Sampler<B> where B: hal::Backend {
	fn drop(&mut self) {
		if !self.resources_destroyed {
			self.destroy_resources();
		}
	}
}