/*!
 * Wrapper around MemoryBufferBuilder that
 * builds a data buffer for 2D textures.
 * This buffer is for uploading the color data from
 * disk to GPU; to actually use
 * the texture for rendering,
 * use a pipeline::Image.
 */
use std::fs::File;

use super::BufferBuilder;
use math::Size;

use failure::Error;

pub struct ImageBufferBuilder {
	buffer_builder: BufferBuilder,
	image_dimensions: Size,
	//The file to load from disk.
	//if None, the resulting buffer will be
	//cleared to 0.
	image_file: Option<File>
}

pub impl ImageBufferBuilder {
	pub fn buffer_builder(&mut self) -> &mut BufferBuilder { self.buffer_builder }

	pub fn build(&self) -> Result<MemoryBuffer, Error> {
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

	pub fn with_image_file(&mut self, value: File) -> &mut ImageBufferBuilder {
		unimplemented!();
		self.image_file = Some(value);
		unimplemented!();
		// self.image_dimensions = ?;

		self
	}
}