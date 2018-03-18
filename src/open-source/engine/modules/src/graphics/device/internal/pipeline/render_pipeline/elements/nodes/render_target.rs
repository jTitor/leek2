/*!
 * Defines the RenderTarget struct.
 */
use graphics::device::internal::pipeline::DeviceResource;
use math::screen::Size;

use std::marker::PhantomData;
use std::rc::Weak;

use gfx_hal as hal;
use gfx_hal::Device;
use gfx_hal::{format as f, device as d, image as i};
use failure::Error;

/**
 * Abstracts memory buffers that rendering data
 * can be written to.
 */
pub struct RenderTarget<B: hal::Backend> {
	//TODO_rust: impl fields
	framebuffers: Vec<B::Framebuffer>,
	frame_images: Vec<(B::Image, B::ImageView)>,
	/**
	 * The RenderTarget's id in the
	 * DeviceController's buffer list.
	 */
	rt_device_id: usize,
	resources_destroyed_val: bool
}

impl<B: hal::Backend> RenderTarget<B> {
	fn mark_destroyed(&mut self) {
		debug_assert!(!self.resources_destroyed_val,
			"RenderTarget already marked as destroyed");

		self.resources_destroyed_val = true;
	}
}

impl<B: hal::Backend> Drop for RenderTarget<B> {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed_val, "MemoryBuffer went out of scope without having its memory destroyed");
	}
}

impl<B: hal::Backend> DeviceResource<B> for RenderTarget<B> {
	fn get_resource(device: &B::Device) -> Weak<&Self> {
		unimplemented!()
	}

	fn destroy_resource(&mut self, device: &B::Device) -> Result<(), Error> {
		for framebuffer in self.framebuffers {
			device.destroy_framebuffer(framebuffer);
		}

		for (image, rtv) in self.frame_images {
			device.destroy_image_view(rtv);
			device.destroy_image(image);
		}
		unimplemented!();

		self.mark_destroyed();

		Ok(())
	}

	fn resources_destroyed(&self) -> bool {
		self.resources_destroyed_val
	}
}

pub struct RenderTargetBuilder<B> where B: hal::Backend {
	_backend_type: PhantomData<B>
}
impl<B: hal::Backend> RenderTargetBuilder<B> {
	pub fn from_gfx_backbuffer(device: &B::Device, backbuffer: &hal::Backbuffer<B>, render_pass: &B::RenderPass, surface_format: f::Format, image_dims: Size, color_range: i::SubresourceRange) -> RenderTarget<B> {
		//Pull render texture and framebuffer objects
		//from the backbuffer struct we've been given.
		//Exactly what we get depends on the
		//backbuffer's type:
		let (frame_images, framebuffers) = match *backbuffer {
			//A set of textures can be unwrapped
			//to the underlying render textures.
			//The FBOs can be generated on the side.
			hal::Backbuffer::Images(images) => {
				let extent = d::Extent { width: image_dims.width() as _, height: image_dims.height() as _, depth: 1 };
				let pairs = images
					.into_iter()
					.map(|image| {
						let rtv = device.create_image_view(&image, surface_format, f::Swizzle::NO, color_range.clone()).unwrap();
						(image, rtv)
					})
					.collect::<Vec<_>>();
				let fbos = pairs
					.iter()
					.map(|&(_, ref rtv)| {
						device.create_framebuffer(render_pass, Some(rtv), extent).unwrap()
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