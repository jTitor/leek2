/*!
 * Defines the Pipeline struct.
 */
use super::{DescriptorPool, elements};
use graphics::device::internal::pipeline::DeviceResource;

use std::rc::Weak;

use failure::Error;
use gfx_hal as hal;
use gfx_hal::Device;
use gfx_hal::command;

/**
 * Contains the entire rendering pipeline.
 */
pub struct Pipeline<'a, B: hal::Backend> {
	descriptor_pool: DescriptorPool<B>,
	render_passes: Vec<elements::Pass<B>>,
	subpass_pipelines: Vec<elements::SubpassPipeline<'a, B>>,
	resources_destroyed_val: bool
}

impl<'a, B: hal::Backend> Pipeline<'a, B> {
	/**
	 * Generates a submission given a command buffer.
	 */
	pub fn submission_with_cmd_buffer<C, S>(&mut self, cmd_buffer: command::CommandBuffer<B, C, S>) -> Result<(), Error> where S: command::Shot {
		self.submission_callback(cmd_buffer)
	}

	pub fn mark_destroyed(&mut self) {
		debug_assert!(!self.resources_destroyed_val, "resources appear to have already been destroyed once");

		self.resources_destroyed_val = true;
	}
}

impl<'a, B: hal::Backend> Drop for Pipeline<'a, B> {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed_val, "MemoryBuffer went out of scope without having its memory destroyed");
	}
}

impl<'a, B: hal::Backend> DeviceResource<B> for Pipeline<'a, B> {
	fn get_resource(device: &B::Device) -> Weak<&Self> {
		unimplemented!();
	}

	fn destroy_resource(&mut self, device: &B::Device) -> Result<(), Error> {
		unimplemented!();
		device.destroy_descriptor_pool(self.descriptor_pool);
		
		//Destroy all of the pipelines first,
		//since they depend on the render passes.
		for subpass_pipeline in self.subpass_pipelines {
			subpass_pipeline.destroy_resource(device);
		}

		//Destroy the render passes.
		for render_pass in self.render_passes {
			device.destroy_render_pass(render_pass);
		}

		self.mark_destroyed();
	}

	fn resources_destroyed(&self) -> bool {
		self.resources_destroyed_val
	}
}