/*!
 * Defines the RenderPipelineBuilder struct.
 */
use super::PipelineBuilderInternal;
use super::super::{DestroyIterOnDrop, DestroyOnDrop};
use graphics::device::internal::pipeline::Pipeline;

use std::marker::PhantomData;

use failure::Error;
use gfx_hal as hal;

/**
 * Basic builder struct for a render pipeline.
 */
pub struct PipelineBuilder<B: hal::Backend> {
	_backend_type: PhantomData<B>
}

impl<B: hal::Backend> PipelineBuilder<B> {
	/**
	 * Builds the RenderPipeline if possible.
	 */
	pub fn build(&self, device: &B::Device) -> Result<Pipeline<B>, Error> {
		unimplemented!();
		//Perform all the necessary calls to
		//generate the pipeline's individual elements:
		
		//Generate the descriptor set layout.
		let descriptor_set_layout = self.build_descriptor_set_layout(device)?;

		//Generate each render pass:
		//TODO: implement PassBuildLayout
		let render_passes = DestroyIterOnDrop::new(Vec::<Pass>::new(), device);
		for render_pass_layout in self.render_pass_layouts {
			let render_pass = self.build_render_pass(render_pass_layout, device)?;

			render_passes.resource_iter.add(render_pass);
		}

		let subpass_pipelines = DestroyIterOnDrop::new(Vec::<Subpass>::new(), device);
		//Generate each subpass pipeline:
		//TODO: implement SubpassPipelineBuildLayout
		for subpass_pipeline_layout in self.subpass_pipeline_layouts {
			let subpass_pipeline = self.build_subpass_pipeline(subpass_pipeline_layout, device)?;
			
			//Add the subpass pipeline to the
			//final pipeline's vec.
			subpass_pipelines.resource_iter.add(subpass_pipeline);
		}

		//Create external-facing descriptors
		//the render calls can bind to.
		// Descriptor pool -
		// this describes how many descriptors
		// can be allocated at any given time
		// and in how many sets of the given layout.
		//let mut desc_pool = device.create_descriptor_pool(1, &[pso::DescriptorRangeDesc, ...]);
		let mut descriptor_pool = DestroyOnDrop::<DescriptorPool<B>>::new(, device);
		let desc_set = descriptor_pool.resource.allocate_set(&descriptor_set_layout);
		unimplemented!();

		//Unwrap the pipeline assets, 
		//and the full pipeline is ready.
		//let pipeline = ...
		unimplemented!();

		//Ok(pipeline)
	}
}