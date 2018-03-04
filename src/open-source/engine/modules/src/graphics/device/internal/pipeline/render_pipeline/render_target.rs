/*!
 * Abstracts memory buffers that rendering data
 * can be written to.
 */
use gfx_hal as hal;

pub struct RenderTarget<B> where B: hal::Backend {
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

impl<B> RenderTarget<B> where B: hal::Backend {
	fn mark_destroyed(&mut self) {
		debug_assert!(!self.resources_destroyed,
			"RenderTarget already marked as destroyed");

		self.resources_destroyed = true;
	}
}

impl<B> Drop for RenderTarget<B> where B: hal::Backend {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed, "MemoryBuffer went out of scope without having its memory destroyed");
	}
}

impl<B> DeviceResource<RenderTarget<B>> for DeviceController<B> where B: hal::Backend {
	fn get_resource(&mut self) -> Weak<&Image<B>> {
		unimplemented!()
	}

	fn destroy_all_resources(&mut self) -> Result<(), Error> {
		unimplemented!()
	}

	fn destroy_resource(&mut self, resource: &mut T) -> Result<(), Error> {
		for framebuffer in resource.framebuffers {
			self.device.destroy_framebuffer(framebuffer);
		}

		for (image, rtv) in resource.frame_images {
			self.device.destroy_image_view(rtv);
			self.device.destroy_image(image);
		}
		unimplemented!()

		resource.mark_destroyed();
	}
}

pub struct RenderTargetBuilder<B> where B: hal::Backend {}
impl<B> RenderTargetBuilder<B> where B: hal::Backend {
	pub fn build() -> RenderTarget<B> {
		//Pull render texture and framebuffer objects
		//from the backbuffer struct we've been given.
		//Exactly what we get depends on the
		//backbuffer's type:
		let (frame_images, framebuffers) = match backbuffer {
			//A set of textures can be unwrapped
			//to the underlying render textures.
			//The FBOs can be generated on the side.
			Backbuffer::Images(images) => {
				let extent = d::Extent { width: pixel_width as _, height: pixel_height as _, depth: 1 };
				let pairs = images
					.into_iter()
					.map(|image| {
						let rtv = device.create_image_view(&image, surface_format, Swizzle::NO, COLOR_RANGE.clone()).unwrap();
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
			Backbuffer::Framebuffer(fbo) => {
				(Vec::new(), vec![fbo])
			}
		};

		unimplemented!();
	}
}