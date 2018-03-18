/*!
 * Defines the RenderPipelineBuilder struct.
 */
use super::{PipelineBuilderInternal, RenderPassLayout, SubpassPipelineLayout};
use super::super::{DestroyIterOnDrop, DestroyOnDrop};
use graphics::device::internal::pipeline::DeviceResource;
use graphics::device::internal::pipeline::render_pipeline::{DescriptorPool, Pipeline, elements, layout};

use std::cmp;
use std::marker::PhantomData;
use std::rc::Rc;

use failure::Error;
use gfx_hal::{self as hal, pso, Backend, Device};

/**
 * Basic builder struct for a render pipeline.
 */
pub struct PipelineBuilder<'a, B: hal::Backend> {
	/**
	 * A list of layouts describing all render passes
	 * used by this pipeline. This describes where
	 * each subpass of the pipeline is used.
	 */
	pub render_pass_layouts: Vec<RenderPassLayout<'a>>,
	/**
	 * A list of layouts describing 
	 * all subpass pipelines used by this
	 * pipeline. This describes how the pipeline performs
	 * element rendering.
	 */
	pub subpass_pipeline_layouts: Vec<SubpassPipelineLayout<'a, B>>,
	/**
	 * The list of descriptor ranges that
	 * will be used to generate the pipeline's
	 * common descriptor pool.
	 */
	pub descriptor_ranges: Vec<layout::DescriptorRangeDescription>,
	/**
	 * The maximum number of descriptor sets
	 * that can be allocated from this
	 * pipeline's descriptor pool at
	 * any one time.
	 * Is always at least 1, and
	 * during the build() call a value of 0
	 * acts the same as a value of 1.
	 */
	pub max_num_descriptor_sets: usize,
	_backend_type: PhantomData<B>
}

impl<'a, B: hal::Backend> Default for PipelineBuilder<'a, B> {
	fn default() -> Self {
		PipelineBuilder::<B> { ..Default::default() }
	}
}

impl<'a, B: hal::Backend> PipelineBuilder<'a, B> {
	pub fn new() -> PipelineBuilder<'a, B> { Default::default() }

	/**
	 * Builds the RenderPipeline if possible.
	 */
	pub fn build(&self, device: &B::Device) -> Result<Pipeline<B>, Error> {
		let clamped_max_num_descriptor_sets = cmp::max(self.max_num_descriptor_sets, 1);
		let device_rc = Rc::new(device);

		let render_passes = Vec::<elements::Pass<B>>::new();
		let subpass_pipelines = Vec::<elements::SubpassPipeline<B>>::new();
		let mut descriptor_pool = DescriptorPool::<B>::new(device.create_descriptor_pool(clamped_max_num_descriptor_sets, self.descriptor_ranges.as_slice()));

		let result = {
			//Generate each render pass:
			for render_pass_layout in self.render_pass_layouts {
				let render_pass = self.build_render_pass(&render_pass_layout, device_rc)?;

				render_passes.push(render_pass);
			}

			//Generate each subpass pipeline:
			for subpass_pipeline_layout in self.subpass_pipeline_layouts {
				let subpass_pipeline = self.build_subpass_pipeline(&subpass_pipeline_layout, device_rc, &render_passes, &mut descriptor_pool)?;
				
				//Add the subpass pipeline to the
				//final pipeline's vec.
				subpass_pipelines.push(subpass_pipeline);
			}

			//Unwrap the pipeline assets, 
			//and the full pipeline is ready.
			Ok(Pipeline::<B> {
				descriptor_pool: descriptor_pool,
				render_passes: render_passes,
				subpass_pipelines: subpass_pipelines,
				resources_destroyed_val: false
			})
		};

		//If there was any error,
		//unload all of the resources
		if let Err(_) = result {
			DeviceResource::destroy_resource_collection(&mut render_passes, device);
			DeviceResource::destroy_resource_collection(&mut subpass_pipelines, device);
			descriptor_pool.destroy_resource(device);
		}

		result
	}
}