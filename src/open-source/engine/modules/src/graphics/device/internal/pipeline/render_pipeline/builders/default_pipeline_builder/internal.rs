/*!
 * Defines the DefaultPipelineBuilderInternal trait.
 */
use super::DefaultPipelineBuilder;
use super::super::{RenderPassLayout, SubpassPipelineLayout};

use std::mem;
use std::rc::Rc;

use gfx_hal as hal;
use gfx_hal::{Device, DescriptorPool};
use gfx_hal::{pso, pass, image as i, format as f};
use failure::Error;

/**
 * Defines internal operations of a DefaultPipelineBuilder.
 */
pub trait DefaultPipelineBuilderInternal<B: hal::Backend> {
	fn create_render_pass_layouts(&self, device: Rc<&B::Device>) -> Result<Vec<RenderPassLayout>, Error>;

	fn create_subpass_pipeline_layouts(&self, device: Rc<&B::Device>) -> Result<Vec<SubpassPipelineLayout<B>>, Error>;
}

impl<B: hal::Backend> DefaultPipelineBuilderInternal for DefaultPipelineBuilder<B> {
	fn create_render_pass_layouts(&self, device: Rc<&B::Device>) -> Result<Vec<RenderPassLayout>, Error> {
		unimplemented!();
	}

	fn create_subpass_pipeline_layouts(&self, device: Rc<&B::Device>) -> Result<Vec<SubpassPipelineLayout<B>>, Error> {
		unimplemented!();
	}
}