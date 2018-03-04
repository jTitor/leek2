/*!
 * Contains the memory and buffer binding
 * representing an image.
 */
use std::rc::Weak;

use failure::Error;
use gfx_hal as hal;

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
pub struct Image<B> where B: hal::Backend {
	image_binding: B::Image,
	image_render_view: B::ImageView,
	image_memory: B::Memory,
	/** The Image's id in the
	 * DeviceController's buffer list.
	 */
	image_device_id: usize,
	unimplemented!()
	resources_destroyed: bool
}

impl<B> Image<B> where B: hal::Backend {
	pub fn build() -> Result<Image<B>, Error> {
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

	pub fn mark_destroyed(&mut self) {
		debug_assert!(!self.resources_destroyed,
			"Image already marked as destroyed");

		self.resources_destroyed = true;
	}
}

impl<B> Drop for Image<B> where B: hal::Backend {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed, "Image went out of scope without having its memory destroyed");
	}
}

impl<B> DeviceResource<Image<B>> for DeviceController<B> where B: hal::Backend {
	fn get_resource(&mut self) -> Weak<&Image<B>> {
		unimplemented!()
	}

	fn destroy_all_resources(&mut self) -> Result<(), Error> {
		unimplemented!()
	}

	fn destroy_resource(&mut self, resource: &mut T) -> Result<(), Error> {
		//TODO_rust: should be part of the write_buffer() call instead?
		// self.device.destroy_buffer(resource.image_upload_buffer);

		self.device.destroy_image(resource.image_binding);

		self.device.destroy_image_view(resource.image_render_view);

		self.device.free_memory(resource.image_memory);
		unimplemented!()

		resource.mark_destroyed();
	}
}