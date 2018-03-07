/*!
 * Defines the RenderPipelineBuilder struct.
 */
use graphics::device::internal::pipeline::RenderPipeline;

use std::marker::PhantomData;

use failure::Error;
use gfx_hal as hal;

/**
 * Basic builder struct for a render pipeline.
 */
pub struct RenderPipelineBuilder<B: hal::Backend> {
	_backend_type: PhantomData<B>
}

impl<B: hal::Backend> RenderPipelineBuilder<B> {

	/**
	 * Builds the RenderPipeline if possible.
	 */
	pub fn build(&self) -> Result<RenderPipeline<B>, Error> {
		unimplemented!();
	}
}