/*!
 * Defines the default Pipeline
 * builder.
 */
use graphics::device::internal::pipeline::Pipeline;
use super::DefaultPipelineBuilderInternal;
use super::super::PipelineBuilder;

use std::marker::PhantomData;
use std::rc::Rc;

use gfx_hal::{self as hal, format as f};
use failure::Error;

/**
 * Default pipeline builder.
 * This generates a pipeline with one vertex shader
 * and one fragment shader.
 */
#[derive(Default)]
pub struct DefaultPipelineBuilder<B: hal::Backend> {
	_backend_type: PhantomData<B>
}

impl<B: hal::Backend> DefaultPipelineBuilder<B> {
	pub fn new() -> DefaultPipelineBuilder<B> { DefaultPipelineBuilder { ..Default::default() } }

	pub fn build(&self, device: &B::Device, surface_format: f::Format) -> Result<Pipeline<B>, Error> {
		let device_rc = Rc::new(device);

		let mut pipeline_builder = PipelineBuilder::<B>::new();

		//The pipeline builder will handle everything;
		//generate our desired pipeline layout...
		pipeline_builder.render_pass_layouts = self.create_render_pass_layouts(device_rc, surface_format)?;
		pipeline_builder.subpass_pipeline_layouts = self.create_subpass_pipeline_layouts(device_rc)?;

		//...and let the builder rip
		pipeline_builder.build(&device_rc)
	}
}