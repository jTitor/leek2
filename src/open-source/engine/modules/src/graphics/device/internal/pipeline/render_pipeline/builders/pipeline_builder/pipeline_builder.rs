/*!
 * Defines the RenderPipelineBuilder struct.
 */
use super::{PipelineBuilderInternal, RenderPassLayout, SubpassPipelineLayout};
use super::super::{DestroyIterOnDrop, DestroyOnDrop};
use graphics::device::internal::pipeline::render_pipeline::{Pipeline, elements, layout};

use std::marker::PhantomData;
use std::rc::Rc;

use failure::Error;
use gfx_hal::{self as hal, pso};

/**
 * Basic builder struct for a render pipeline.
 */
#[derive(Default)]
pub struct PipelineBuilder<'a, B: hal::Backend> {
	pub render_pass_layouts: Vec<RenderPassLayout<'a>>,
	pub subpass_pipeline_layouts: Vec<SubpassPipelineLayout<'a, B>>,
	_backend_type: PhantomData<B>
}

impl<'a, B: hal::Backend> PipelineBuilder<'a, B> {
	fn new() -> PipelineBuilder<'a, B> { Self::default() }

	/**
	 * Builds the RenderPipeline if possible.
	 */
	pub fn build(&self, device: &B::Device) -> Result<Pipeline<B>, Error> {
		let device_rc = Rc::new(device);

		//Perform all the necessary calls to
		//generate the pipeline's individual elements:
		
		//Generate the descriptor set layout.
		let descriptor_set_layout = self.build_descriptor_set_layout(device_rc)?;

		//Create the pipeline layout.
		unimplemented!();

		//Generate each render pass:
		let render_passes = DestroyIterOnDrop::new(Vec::<elements::Pass<B>>::new(), device_rc);
		for render_pass_layout in self.render_pass_layouts {
			let render_pass = self.build_render_pass(render_pass_layout, device_rc)?;

			render_passes.resource_iter().push(render_pass);
		}

		let subpass_pipelines = DestroyIterOnDrop::new(Vec::<elements::SubpassPipeline<B>>::new(), device_rc);
		//Generate each subpass pipeline:
		for subpass_pipeline_layout in self.subpass_pipeline_layouts {
			let subpass_pipeline = self.build_subpass_pipeline(subpass_pipeline_layout, device_rc)?;
			
			//Add the subpass pipeline to the
			//final pipeline's vec.
			subpass_pipelines.resource_iter().push(subpass_pipeline);
		}

		//Create external-facing descriptors
		//the render calls can bind to.
		// Descriptor pool -
		// this describes how many descriptors
		// can be allocated at any given time
		// and in how many sets of the given layout.
		let mut descriptor_pool_raw = device.create_descriptor_pool(1, &[pso::DescriptorRangeDesc, ...]);
		let mut descriptor_pool = DestroyOnDrop::<DescriptorPool<B>>::new(descriptor_pool_raw, device_rc);

		let desc_set = descriptor_pool.resource_mut().allocate_set(&descriptor_set_layout);

		//Unwrap the pipeline assets, 
		//and the full pipeline is ready.
		Ok(elements::Pipeline::<B> {
			descriptor_pool: descriptor_pool.unwrap(),
			render_passes: render_passes.unwrap(),
			subpass_pipelines: subpass_pipelines.unwrap(),
			resources_destroyed: false
		})
	}
}