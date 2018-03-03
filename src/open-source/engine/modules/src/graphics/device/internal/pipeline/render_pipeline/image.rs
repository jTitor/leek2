/*!
 * Contains the memory and buffer binding
 * representing an image.
 */
use failure::Error;

/**
 * Images consist of three main parts:
 * the GPU-side buffer, the binding
 * used to link it to other pipeline elements,
 * and the render target/view that lets
 * samplers sample it.
 * 
 * This doesn't handle the CPU or disk-side
 * data; other modules must get the data and
 * upload it with Image::write_buffer().
 */
pub struct Image {
	image_binding: ?,
	image_render_view: ?,
	image_memory: ?,
	/** The Image's id in the
	 * DeviceController's buffer list.
	 */
	image_device_id: usize,
	unimplemented!()
	resources_destroyed: bool
}

impl Image {
	pub fn build() -> Result<Image, Error> {
		//Build the buffer first...
		//The sample code sets Usage::TRANSFER_DST
		//so the image can be uploaded from CPU to GPU,
		//and Usage::SAMPLED so the sampler can get at it
			// let image_unbound = device.create_image(kind, 1, ColorFormat::SELF, i::Usage::TRANSFER_DST | i::Usage::SAMPLED).unwrap();
		//Requirements are a generic opaque enum that
		//gets used to filter available memory types
		//to one the image buffer can use.
			// let image_req = device.get_image_requirements(&image_unbound);

			// let device_type = memory_types
			// 	.iter()
			// 	.enumerate()
			// 	.position(|(id, memory_type)| {
		//NOTE: properties of this memory
		//are different from FileBuffer or MemoryBuffer.
			// 		image_req.type_mask & (1 << id) != 0 &&
			// 		memory_type.properties.contains(m::Properties::DEVICE_LOCAL)
			// 	})
			// 	.unwrap()
			// 	.into();
		//Allocate the actual memory here...
			// let image_memory = device.allocate_memory(device_type, image_req.size).unwrap();

		//Create the binding...
		let image_binding = device.bind_image_memory(&image_memory, 0, image_unbound)?;

		//And make the render view.
		let image_render_view = device.create_image_view(&image_binding, ColorFormat::SELF, Swizzle::NO, COLOR_RANGE.clone())?;

		unimplemented!()
	}

	pub fn write_buffer(&mut self, data: ?) -> Result<(), Error> {
		unimplemented!()
	}

	pub fn destroy_resources(&mut self) {
		debug_assert!(!self.resources_destroyed);

		if !self.resources_destroyed {
			//TODO_rust: should be part of the write_buffer() call instead?
			// self.device.destroy_buffer(self.image_upload_buffer);

			self.device.destroy_image(self.image_binding);

			self.device.destroy_image_view(self.image_render_view);

			self.device.free_memory(image_memory);

			self.resources_destroyed = true;
		}
	}
}

impl Drop for Image {
	fn drop(&mut self) {
		if !self.resources_destroyed {
			self.destroy_resources();
		}
	}
}