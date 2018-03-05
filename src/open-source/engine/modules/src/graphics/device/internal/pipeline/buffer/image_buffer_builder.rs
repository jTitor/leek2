/*!
 * Wrapper around MemoryBufferBuilder that
 * builds a data buffer for 2D textures.
 * This buffer is for uploading the color data from
 * disk to GPU; to actually use
 * the texture for rendering,
 * use a pipeline::Image.
 */
use std::fs::File;

use super::{BufferBuilder, MemoryBuffer};
use math::screen::Size;

use gfx_hal as hal;
use failure::Error;

pub struct ImageBufferBuilder<B: hal::Backend> {
	buffer_builder: BufferBuilder<B>,
	image_dimensions: Size,
	//The file to load from disk.
	//if None, the resulting buffer will be
	//cleared to 0.
	image_file: Option<File>
}

impl<B: hal::Backend> ImageBufferBuilder<B> {
	pub fn buffer_builder(&mut self) -> &mut BufferBuilder<B> { self.buffer_builder }

	pub fn build(&self) -> Result<MemoryBuffer<B>, Error> {
		//TODO:
		//Convert dimensions into
		//buffer size.
		unimplemented!();

		//TODO: If there's a file specified,
		//load its data and upload it to the
		//buffer.
		if let Some(image_data) = self.image_file {
			unimplemented!();
		}

		Ok(unimplemented!())
	}

	pub fn with_image_file(&mut self, value: File) -> &mut ImageBufferBuilder<B> {
		unimplemented!();
		self.image_file = Some(value);
		unimplemented!();
		// self.image_dimensions = ?;

		self
	}
}