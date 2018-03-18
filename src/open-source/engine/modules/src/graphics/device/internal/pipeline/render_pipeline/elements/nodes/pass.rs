/*!
 * Defines the RenderPass struct.
 */
use graphics::device::internal::pipeline::DeviceResource;

use std::rc::Weak;

use failure::Error;
use gfx_hal::{self as hal, Backend};

/**
 * Encapsulates a top-level rendering pass.
 * Contains at least one Subpass, which can only be
 * run within this pass.
 */
pub struct Pass<B: hal::Backend> {
	pub render_pass_impl: B::RenderPass,
	resources_destroyed_val: bool
}

impl<B: hal::Backend> Pass<B> {
	pub fn new(render_pass_impl: B::RenderPass) -> Pass<B> {
		Pass::<B> {
			render_pass_impl: render_pass_impl,
			resources_destroyed_val: false
		}
	}
}

impl<B: hal::Backend> DeviceResource<B> for Pass<B> {
	fn get_resource(device: &B::Device) -> Weak<&Self> {
		unimplemented!();
	}

	fn destroy_resource(&mut self, device: &B::Device) -> Result<(), Error> {
		device.destroy_render_pass(self.render_pass_impl);
		self.resources_destroyed_val = true;

		Ok(())
	}

	fn resources_destroyed(&self) -> bool {
		self.resources_destroyed_val
	}
}