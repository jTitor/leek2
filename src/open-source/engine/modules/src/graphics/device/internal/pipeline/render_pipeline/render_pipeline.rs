/*!
 * Performs the actual rendering for a Device.
 * A DeviceController uses this pipeline to decide what
 * draw calls must be executed.
 */
use graphics::device::internal::pipeline::{DeviceController, DeviceResource};

use std::rc::Weak;

use failure::Error;
use gfx_hal as hal;
use gfx_hal::Device;
use gfx_hal::command;

pub struct RenderPipeline<B> where B: hal::Backend {
	//The exact backend doesn't matter,
	//so much as how data is submitted to it.

	/**
	 * Contains all descriptors used by this pipeline.
	 */
	desc_pool: B::DescriptorPool,
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
	 * The (currently sole) render pass used by the
	 * pipeline.
	 */
	render_pass: B::RenderPass,
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

impl<B> RenderPipeline<B> where B: hal::Backend {
	/**
	 * Generates a submission given a command buffer.
	 */
	pub fn submission_with_cmd_buffer<C, S>(&mut self, cmd_buffer: command::CommandBuffer<B, C, S>) -> Result<(), Error> where S: command::Shot {
		self.submission_callback(cmd_buffer)
	}

	pub fn mark_destroyed(&mut self) {
		debug_assert!(!self.resources_destroyed, "resources appear to have already been destroyed once");

		self.resources_destroyed = true;
	}
}

impl<B> Drop for RenderPipeline<B> where B: hal::Backend {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed, "MemoryBuffer went out of scope without having its memory destroyed");
	}
}

pub trait RenderPipelineCapability {}
impl<B: hal::Backend> RenderPipelineCapability for RenderPipeline<B> {}

impl<B: hal::Backend, C: RenderPipelineCapability> DeviceResource<C> for DeviceController<B> {
	fn get_resource<C>(&mut self) -> Weak<&C> {
		unimplemented!();
	}

	fn destroy_all_resources<C>(&mut self) -> Result<(), Error> {
		// for pipeline in self.resource_lists.pipelines {
		// 	self.device.destroy_pipeline_layout(unimplemented!());
		// 	self.device.destroy_render_pass(unimplemented!());
		// 	self.device.destroy_graphics_pipeline(pipeline);
		// }
		unimplemented!();

		Ok(())
	}

	fn destroy_resource<C>(&mut self, resource: &mut C) -> Result<(), Error> {
		unimplemented!();
		self.device.destroy_descriptor_pool(resource.desc_pool);
		self.device.destroy_descriptor_set_layout(resource.set_layout);
		self.device.destroy_pipeline_layout(resource.pipeline_layout);
		self.device.destroy_render_pass(resource.render_pass);
		self.device.destroy_graphics_pipeline(resource.pipeline_impl);

		resource.mark_destroyed();
	}
}