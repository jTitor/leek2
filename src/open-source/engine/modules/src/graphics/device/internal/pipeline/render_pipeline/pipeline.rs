/*!
 * Defines the Pipeline struct.
 */
use super::super::DescriptorPool;
use super::elements;
use graphics::device::internal::pipeline::DeviceResource;

use std::marker::PhantomData;

use gfx_hal as hal;

/**
 * Contains the entire rendering pipeline.
 */
pub struct Pipeline<B: hal::Backend> {
	descriptor_pool: DescriptorPool<B>,
	render_passes: Vec<elements::Pass<B>>,
	subpass_pipelines: Vec<elements::SubpassPipeline<B>>,
	resources_destroyed: bool,
	_backend_type: PhantomData<B>
}

impl<B: hal::Backend> RenderPipeline<B> {
	pub fn mark_destroyed(&mut self) {
		debug_assert!(!self.resources_destroyed, "resources appear to have already been destroyed once");

		self.resources_destroyed = true;
	}
}

impl<B: hal::Backend> Drop for RenderPipeline<B> {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed, "MemoryBuffer went out of scope without having its memory destroyed");
	}
}

impl<B: hal::Backend> DeviceResource<B> for RenderPipeline<B> {
	fn get_resource(device: &mut B::Device) -> Weak<&Self> {
		unimplemented!();
	}

	fn destroy_all_resources(device: &mut B::Device, resource_list: &Vec<Self>) -> Result<(), Error> {
		// for pipeline in self.resource_lists.pipelines {
		// 	self.device.destroy_pipeline_layout(unimplemented!());
		// 	self.device.destroy_render_pass(unimplemented!());
		// 	self.device.destroy_graphics_pipeline(pipeline);
		// }
		unimplemented!();

		Ok(())
	}

	fn destroy_resource(device: &mut B::Device, resource: &mut Self) -> Result<(), Error> {
		unimplemented!();
		device.destroy_descriptor_pool(resource.descriptor_pool);
		
		//Destroy all of the pipelines first,
		//since they depend on the render passes.
		for subpass_pipeline in resource.subpass_pipelines {
			DeviceResource::<SubpassPipeline<B>>::destroy_resource(device, subpass_pipeline);
		}

		//Destroy the render passes.
		for render_pass in resource.render_passes {
			device.destroy_render_pass(resource.render_pass);
		}

		resource.mark_destroyed();
	}
}