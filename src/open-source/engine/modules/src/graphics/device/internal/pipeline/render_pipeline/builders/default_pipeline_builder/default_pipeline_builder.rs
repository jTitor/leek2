/*!
 * Defines the default RenderPipeline
 * builder.
 */
use graphics::device::internal::pipeline::Pipeline;
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
	_backend_type: PhantomData<B>
}

impl<B> DefaultPipelineBuilder<B> where B: hal::Backend {
	pub fn new() -> DefaultPipelineBuilder<B> { Self::default() }

	pub fn build(&self, device: &mut B::Device, surface_format: f::Format) -> Result<Pipeline<B>, Error> {
		let device_rc = Rc::new(device);

		let mut pipeline_builder = RenderPipelineBuilder::<B>::new();

		//The pipeline builder will handle everything;
		//generate our desired pipeline layout...
		pipeline_builder.render_pass_layouts = self.create_render_pass_layouts(device_rc)?;
		pipeline_builder.subpass_pipeline_layouts = self.create_subpass_pipeline_layouts(device_rc)?;

		//...and let the builder rip
		pipeline_builder.build(device_rc)
	}
}