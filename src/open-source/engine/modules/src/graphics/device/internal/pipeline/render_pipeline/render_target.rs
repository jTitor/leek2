/*!
 * Abstracts memory buffers that rendering data
 * can be written to.
 */
pub struct RenderTarget {
	//TODO_rust: impl fields
}

impl RenderTarget {
	fn destroy_resources(&mut self) {
		debug_assert!(!self.resources_destroyed);
		if !self.resources_destroyed {
			for framebuffer in self.framebuffers {
				self.device.destroy_framebuffer(framebuffer);
			}

			for (image, rtv) in self.frame_images {
				self.device.destroy_image_view(rtv);
				self.device.destroy_image(image);
			}

			self.resources_destroyed = true;
		}
	}
}

impl Drop for RenderTarget {
	fn drop(&mut self) {
		if !self.resources_destroyed {
			self.destroy_resources();
		}
	}
}

pub struct RenderTargetBuilder {}
impl RenderTargetBuilder {
	pub fn build() -> RenderTarget {
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