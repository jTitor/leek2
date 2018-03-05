/*!
 * Abstracts memory buffers that rendering data
 * can be written to.
 */
use graphics::device::internal::pipeline::{DeviceController, DeviceResource};

use std::rc::Weak;

use gfx_hal as hal;
use gfx_hal::{format as f, device as d};
use failure::Error;

pub struct RenderTarget<B: hal::Backend> {
	//TODO_rust: impl fields
	framebuffers: Vec<B::Framebuffer>,
	frame_images: Vec<(B::Image, B::ImageView)>,
	/**
	 * The RenderTarget's id in the
	 * DeviceController's buffer list.
	 */
	rt_device_id: usize,
	resources_destroyed: bool
}

impl<B: hal::Backend> RenderTarget<B> {
	fn mark_destroyed(&mut self) {
		debug_assert!(!self.resources_destroyed,
			"RenderTarget already marked as destroyed");

		self.resources_destroyed = true;
	}
}

impl<B: hal::Backend> Drop for RenderTarget<B> {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed, "MemoryBuffer went out of scope without having its memory destroyed");
	}
}

pub trait RenderTargetCapability {}
impl<B: hal::Backend> RenderTargetCapability for RenderTarget<B> {}

impl<B: hal::Backend, C: RenderTargetCapability> DeviceResource<C> for DeviceController<B> {
	fn get_resource(&mut self) -> Weak<&C> {
		unimplemented!()
	}

	fn destroy_all_resources<C>(&mut self) -> Result<(), Error> {
		// for render_target in self.resource_lists.render_targets {
		// 	device.destroy_image_view(render_target);
		// 	//TODO: In the example, the RTs
		// 	//have a backing image; RTs should have
		// 	//a separate image list?
		// 	//device.destroy_image(image);
		// }
		unimplemented!();

		Ok(())
	}

	fn destroy_resource<C>(&mut self, resource: &mut C) -> Result<(), Error> {
		for framebuffer in resource.framebuffers {
			self.device.destroy_framebuffer(framebuffer);
		}

		for (image, rtv) in resource.frame_images {
			self.device.destroy_image_view(rtv);
			self.device.destroy_image(image);
		}
		unimplemented!();

		resource.mark_destroyed();

		Ok(())
	}
}

pub struct RenderTargetBuilder<B> where B: hal::Backend {}
impl<B: hal::Backend> RenderTargetBuilder<B> {
	pub fn build(device: &mut hal::Device) -> RenderTarget<B> {
		//Pull render texture and framebuffer objects
		//from the backbuffer struct we've been given.
		//Exactly what we get depends on the
		//backbuffer's type:
		let (frame_images, framebuffers) = match backbuffer {
			//A set of textures can be unwrapped
			//to the underlying render textures.
			//The FBOs can be generated on the side.
			hal::Backbuffer::Images(images) => {
				let extent = d::Extent { width: pixel_width as _, height: pixel_height as _, depth: 1 };
				let pairs = images
					.into_iter()
					.map(|image| {
						let rtv = device.create_image_view(&image, surface_format, f::Swizzle::NO, COLOR_RANGE.clone()).unwrap();
						(image, rtv)
					})
					.collect::<Vec<_>>();
				let fbos = pairs
					.iter()
					.map(|&(_, ref rtv)| {
						device.create_framebuffer(&render_pass, Some(rtv), extent).unwrap()
					})
					.collect();
				(pairs, fbos)
			}
			//A straight framebuffer has no
			//render textures, so just
			//return the FBO inside it
			hal::Backbuffer::Framebuffer(fbo) => {
				(Vec::new(), vec![fbo])
			}
		};

		unimplemented!();
	}
}