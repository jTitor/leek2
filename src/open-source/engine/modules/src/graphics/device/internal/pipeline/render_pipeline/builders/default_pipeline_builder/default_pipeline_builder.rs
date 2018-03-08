/*!
 * Defines the default RenderPipeline
 * builder.
 */
use graphics::device::internal::pipeline::RenderPipeline;
use super::DefaultPipelineBuilderInternal;
use super::super::RenderPipelineBuilder;

use std::marker::PhantomData;
use std::rc::Rc;

use gfx_hal as hal;
use failure::Error;

/**
 * Default pipeline builder.
 * This generates a pipeline with one vertex shader
 * and one fragment shader.
 */
#[derive(Default)]
pub struct DefaultPipelineBuilder<B> where B: hal::Backend {
	pipeline_builder: RenderPipelineBuilder<B>,
	_backend_type: PhantomData<B>
}

impl<B> DefaultPipelineBuilder<B> where B: hal::Backend {
	pub fn new() -> DefaultPipelineBuilder<B> { Self::default() }

	pub fn build(&self, device: &mut B::Device, surface_format: f::Format) -> Result<RenderPipeline<B>, Error> {
		let device_rc = Rc::new(device);

		self.build_pipeline(device_rc, surface_format)
	}
}