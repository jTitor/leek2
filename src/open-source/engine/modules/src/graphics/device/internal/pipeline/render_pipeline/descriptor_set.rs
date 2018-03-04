/*!
 * Binds shader descriptors to input views/samplers.
 */
use gfx_hal as hal;
use hal::pso;

pub struct DescriptorSet {
	unimplemented!()
}

impl DescriptorSet {
	pub fn connect(&mut self) {
		//TODO_rust: generalize from demo code
			device.write_descriptor_sets::<_, Range<_>>(vec![
			pso::DescriptorSetWrite {
				set: &desc_set,
				binding: 0,
				array_offset: 0,
				write: pso::DescriptorWrite::SampledImage(&[(&image_srv, i::ImageLayout::Undefined)]),
			},
			pso::DescriptorSetWrite {
				set: &desc_set,
				binding: 1,
				array_offset: 0,
				write: pso::DescriptorWrite::Sampler(&[&sampler]),
			},
		]);
	}
}