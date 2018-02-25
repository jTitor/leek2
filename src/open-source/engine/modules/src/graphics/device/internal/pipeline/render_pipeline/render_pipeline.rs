/*!
 * Performs the actual rendering for a Device.
 * A DeviceController uses this pipeline to decide what
 * draw calls must be executed.
 */
use failure::Error;

pub struct RenderPipeline {
	//The exact backend doesn't matter,
	//so much as how data is submitted to it.
}

impl RenderPipeline {
	pub fn destroy_resources(&mut self) {
		debug_assert!(!self.resources_destroyed);

		if !self.resources_destroyed {
			self.device.destroy_descriptor_pool(self.desc_pool);
			self.device.destroy_descriptor_set_layout(self.set_layout);
			self.device.destroy_pipeline_layout(self.pipeline_layout);
			self.device.destroy_render_pass(self.render_pass);
			self.device.destroy_graphics_pipeline(self.pipeline);

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