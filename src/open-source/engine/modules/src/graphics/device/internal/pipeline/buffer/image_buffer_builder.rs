/*!
 * Defines the ImageBufferBuilder struct.
 */
use super::{BufferBuilder, MemoryBuffer};
use math::screen::Size;

use std::fs::File;
use std::marker::PhantomData;

use gfx_hal as hal;
use failure::Error;

/**
 * Wrapper around MemoryBufferBuilder that
 * builds a data buffer for 2D textures.
 * 
 * This buffer is for uploading the color data from
 * disk to GPU; to actually use
 * the texture for rendering,
 * use a pipeline::Image.
 */
pub struct ImageBufferBuilder<B: hal::Backend> {
	buffer_builder: BufferBuilder<B>,
	image_dimensions: Size,
	/**
	 * The file to load from disk.
	 * If None, the resulting buffer will be
	 * cleared to 0.
	 */
	image_file: Option<File>,
	_backend_type: PhantomData<B>
}

impl<B: hal::Backend> ImageBufferBuilder<B> {
	pub fn buffer_builder(&mut self) -> &mut BufferBuilder<B> { &mut self.buffer_builder }

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