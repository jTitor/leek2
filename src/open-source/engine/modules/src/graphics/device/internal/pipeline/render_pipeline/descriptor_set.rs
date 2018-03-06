/*!
 * Binds shader descriptors to input views/samplers.
 */
use std::marker::PhantomData;
use std::ops::Range;

use gfx_hal as hal;
use gfx_hal::device::Device;
use gfx_hal::{pso, device as d, image as i};

pub struct DescriptorSet<B: hal::Backend> {
	_backend_type: PhantomData<B>
}

impl<B: hal::Backend> DescriptorSet<B> {
	pub fn connect<C>(&mut self, device: &C, sampler: &B::Sampler, image_srv: &B::ImageView, desc_set: &B::DescriptorSet) where C: d::Device<B> {
		//TODO_rust: generalize from demo code
		device.write_descriptor_sets::<_, Range<_>>(vec![
			pso::DescriptorSetWrite {
				set: desc_set,
				binding: 0,
				array_offset: 0,
				write: pso::DescriptorWrite::SampledImage(vec![(image_srv, i::ImageLayout::Undefined)]),
			},
			pso::DescriptorSetWrite {
				set: desc_set,
				binding: 1,
				array_offset: 0,
				write: pso::DescriptorWrite::Sampler(vec![sampler]),
			},
		]);
	}
}