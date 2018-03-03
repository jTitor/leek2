/*!
 * Samples color data from a texture.
 */

pub struct Sampler {
	unimplemented!()
	/** The Sampler's id in the
	 * DeviceController's buffer list.
	 */
	sampler_device_id: usize,
	resources_destroyed: bool
}

impl Sampler {
	pub fn build() -> Sampler {
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

impl Drop for Sampler {
	fn drop(&mut self) {
		if !self.resources_destroyed {
			self.destroy_resources();
		}
	}
}