/*!
 * Stores the resource lists used
 * by the DeviceController.
 */
use gfx_hal as hal;
use graphics::device::internal::pipeline as pipeline;

pub struct DeviceResourceLists<B> where B: hal::Backend {
	buffers: Vec<pipeline::Buffer<B>>,
	images: Vec<pipeline::Image<B>>,
	render_targets: Vec<pipeline::RenderTarget<B>>,
	pipelines: Vec<pipeline::RenderPipeline<B>>,
	samplers: Vec<pipeline::Sampler<B>>
}