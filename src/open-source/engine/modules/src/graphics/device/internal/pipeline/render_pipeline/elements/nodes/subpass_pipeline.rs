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
	 * An instance of the descriptor set
	 * described by ```set_layout```.
	 */
	descriptor_set: B::DescriptorSet,
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

	resources_destroyed_val: bool
}

impl<'a, B> SubpassPipeline<'a, B> where B: hal::Backend {
	pub fn mark_destroyed(&mut self) {
		debug_assert!(!self.resources_destroyed_val, "resources appear to have already been destroyed once");

		self.resources_destroyed_val = true;
	}
}

impl<'a, B> Drop for SubpassPipeline<'a, B> where B: hal::Backend {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed_val, "MemoryBuffer went out of scope without having its memory destroyed");
	}
}

impl<'a, B: hal::Backend> DeviceResource<B> for SubpassPipeline<'a, B> {
	fn get_resource(device: &B::Device) -> Weak<&Self> {
		unimplemented!();
	}

	fn destroy_resource(&mut self, device: &B::Device) -> Result<(), Error> {
		unimplemented!();
		device.destroy_descriptor_set_layout(self.set_layout);
		device.destroy_pipeline_layout(self.pipeline_layout);
		device.destroy_graphics_pipeline(self.pipeline_impl);

		//The descriptor set instance
		//will be freed when the
		//descriptor pool is destroyed,
		//so there's no need (or way) to destroy
		//it here

		self.mark_destroyed();

		Ok(())
	}

	fn resources_destroyed(&self) -> bool {
		self.resources_destroyed_val
	}
}