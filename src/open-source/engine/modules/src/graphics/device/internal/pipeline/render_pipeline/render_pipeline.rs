/*!
 * Performs the actual rendering for a Device.
 * A DeviceController uses this pipeline to decide what
 * draw calls must be executed.
 */
use gfx_hal as hal;
use failure::Error;

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
	submission_callback: ?,

	resources_destroyed: bool
}

impl<B> RenderPipeline<B> where B: hal::Backend {
	/**
	 * Generates a submission given a command buffer.
	 */
	pub fn submission_with_cmd_buffer<C, S>(&mut self, cmd_buffer: CommandBuffer<B, C, S>) -> Result<(), Error> where S: hal::Shot {
		self.submission_callback(cmd_buffer)
	}

	pub fn destroy_resources(&mut self) {
		debug_assert!(!self.resources_destroyed);

		if !self.resources_destroyed {
			self.device.destroy_descriptor_pool(self.desc_pool);
			self.device.destroy_descriptor_set_layout(self.set_layout);
			self.device.destroy_pipeline_layout(self.pipeline_layout);
			self.device.destroy_render_pass(self.render_pass);
			self.device.destroy_graphics_pipeline(self.pipeline_impl);

			self.resources_destroyed = true;
		}
	}
}

impl<B> Drop for RenderPipeline<B> where B: hal::Backend {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed, "MemoryBuffer went out of scope without having its memory destroyed");
	}
}