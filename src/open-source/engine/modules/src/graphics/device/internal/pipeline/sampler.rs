/*!
 * Samples color data from a texture.
 */

pub struct Sampler {
	unimplemented!()
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
}