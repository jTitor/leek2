/*!
 * Defines the DeviceResourceLists struct.
 */
use gfx_hal as hal;
use graphics::device::internal::pipeline as pipeline;
use graphics::device::internal::pipeline::render_pipeline::{elements};

/**
 * Stores the resource lists used
 * by the DeviceController.
 */
pub struct DeviceResourceLists<'a, B> where B: hal::Backend {
	pub buffers: Vec<pipeline::MemoryBuffer<B>>,
	pub images: Vec<elements::Image<B>>,
	pub render_targets: Vec<elements::RenderTarget<B>>,
	pub pipelines: Vec<pipeline::Pipeline<'a, B>>,
	pub samplers: Vec<elements::Sampler<B>>
}

impl<'a, B> Default for DeviceResourceLists<'a, B> where B: hal::Backend {
	fn default() -> Self {
		DeviceResourceLists::<'a, B> {
			buffers: vec!(),
			images: vec!(),
			render_targets: vec!(),
			pipelines: vec!(),
			samplers: vec!()
		}
	}
}