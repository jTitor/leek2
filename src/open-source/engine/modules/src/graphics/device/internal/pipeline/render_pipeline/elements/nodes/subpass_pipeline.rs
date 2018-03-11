/*!
 * Defines the RenderPipeline struct.
 */
use graphics::device::internal::pipeline::DeviceResource;
use super::Subpass;

use std::rc::Weak;

use failure::Error;
use gfx_hal as hal;
use gfx_hal::Device;

/**
 * Performs rendering for a given subpass.
 * 
 * TODO: fix description of following:
 * "Performs the actual rendering for a Device.
 * A DeviceController uses this pipeline to decide what
 * draw calls must be executed."
 */
pub struct SubpassPipeline<'a, B: hal::Backend> {
	//The exact backend doesn't matter,
	//so much as how data is submitted to it.

	/**
	 * Specifies the relation of descriptors to
	 * pipeline attributes.
	 */
	set_layout: B::DescriptorSetLayout,
	/**
	 * Specifies the pipeline's layout.
	 */
	pipeline_layout: B::PipelineLayout,
	/**
	 * The subpass this subpass pipeline
	 * belongs to.
	 */
	subpass: Subpass<'a, B>,
	/**
	 * The actual gfx_hal pipeline.
	 */
	pipeline_impl: B::GraphicsPipeline,
	/**
	 * Used to generate a command queue
	 * submission for a graphics device controller.
	 */
	// submission_callback: ?,

	resources_destroyed: bool
}

impl<'a, B> SubpassPipeline<'a, B> where B: hal::Backend {
	pub fn mark_destroyed(&mut self) {
		debug_assert!(!self.resources_destroyed, "resources appear to have already been destroyed once");

		self.resources_destroyed = true;
	}
}

impl<'a, B> Drop for SubpassPipeline<'a, B> where B: hal::Backend {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed, "MemoryBuffer went out of scope without having its memory destroyed");
	}
}

impl<'a, B: hal::Backend> DeviceResource<B> for SubpassPipeline<'a, B> {
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
		device.destroy_descriptor_set_layout(resource.set_layout);
		device.destroy_pipeline_layout(resource.pipeline_layout);
		device.destroy_graphics_pipeline(resource.pipeline_impl);

		resource.mark_destroyed();
	}

	fn resources_destroyed(&self) -> bool {
		self.resources_destroyed;
	}
}