/*!
 * Stores the resource lists used
 * by the DeviceController.
 */
use gfx_hal as hal;
use graphics::device::internal::pipeline as pipeline;

pub struct DeviceResourceLists<B> where B: hal::Backend {
	pub buffers: Vec<pipeline::Buffer<B>>,
	pub images: Vec<pipeline::Image<B>>,
	pub render_targets: Vec<pipeline::RenderTarget<B>>,
	pub pipelines: Vec<pipeline::RenderPipeline<B>>,
	pub samplers: Vec<pipeline::Sampler<B>>
}

impl Default for DeviceResourceLists<B> where B: hal::Backend {
	fn default() -> Self {
		DeviceResourceLists<B> {
			buffers: vec!(),
			images: vec!(),
			render_targets: vec!(),
			pipelines: vec!(),
			samplers: vec!()
		}
	}
}