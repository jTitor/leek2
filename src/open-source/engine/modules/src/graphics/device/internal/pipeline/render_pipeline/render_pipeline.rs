/*!
 * Performs the actual rendering for a Device.
 * A DeviceController uses this pipeline to decide what
 * draw calls must be executed.
 */
use failure::Error;

pub struct RenderPipeline {
	//The exact backend doesn't matter,
	//so much as how data is submitted to it.

	/**
	 * Contains all descriptors used by this pipeline.
	 */
	desc_pool: ?,
	/**
	 * Specifies the relation of descriptors to
	 * pipeline attributes.
	 */
	set_layout: ?,
	/**
	 * Specifies the pipeline's layout.
	 */
	pipeline_layout: ?,
	/**
	 * The (currently sole) render pass used by the
	 * pipeline.
	 */
	render_pass: ?,
	/**
	 * The actual gfx_hal pipeline.
	 */
	pipeline_impl: ?,
	/**
	 * Used to generate a command queue
	 * submission for a graphics device controller.
	 */
	submission_callback: ?,

	resources_destroyed: bool
}

impl RenderPipeline {
	/**
	 * Generates a submission given a command buffer.
	 */
	pub fn submission_with_cmd_buffer(&mut self, cmd_buffer: ?) -> ? {
		self.submission_callback(cmd_buffer);
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

impl Drop for RenderPipeline {
	fn drop(&mut self) {
		if !self.resources_destroyed {
			self.destroy_resources();
		}
	}
}