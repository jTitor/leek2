/*!
 * Contains the memory and buffer binding
 * representing an image.
 */
use graphics::device::internal::pipeline::{DeviceController, DeviceResource};
use super::ImageInit;

use std::rc::Weak;

use failure::Error;
use gfx_hal as hal;
use gfx_hal::format as f;
use gfx_hal::format::Rgba8Srgb as ColorFormat;

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
pub struct Image<B: hal::Backend> {
	image_binding: B::Image,
	image_render_view: B::ImageView,
	image_memory: B::Memory,
	/** The Image's id in the
	 * DeviceController's buffer list.
	 */
	image_device_id: usize,
	resources_destroyed: bool
}

impl<B: hal::Backend> Image<B> {
	pub fn build(device: &mut hal::Device) -> Result<Image<B>, Error> {
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
		let image_render_view = device.create_image_view(&image_binding, ColorFormat::SELF, f::Swizzle::NO, COLOR_RANGE.clone())?;

		unimplemented!()
	}

	// pub fn write_buffer(&mut self, data: ?) -> Result<(), Error> {
	// 	unimplemented!()
	// }

	pub fn mark_destroyed(&mut self) {
		debug_assert!(!self.resources_destroyed,
			"Image already marked as destroyed");

		self.resources_destroyed = true;
	}

	fn from_file(file_name: String) -> Image<B> {
		Self::load_file()?;

		//It looks like you can't upload
		//directly from CPU to a hal::Image,
		//or at least you have to query
		//for the capability.
		//
		//Instead you make a temporary buffer, then
		//do a buffer to RT copy; this is
		//a GPU->GPU operation so it's always ok.
		Self::create_upload_buffer()?;
		Self::copy_image_to_upload_buffer()?;

		//Now create the *actual* image object.
		Self::create_image_object()?;

		//Copy from the upload buffer to the
		//image object.
	}
}

impl<B: hal::Backend> Drop for Image<B> {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed, "Image went out of scope without having its memory destroyed");
	}
}

pub trait ImageCapability {}
impl<B: hal::Backend> ImageCapability for Image<B> {}

impl<B: hal::Backend, C: ImageCapability> DeviceResource<C> for DeviceController<B> where {
	fn get_resource(&mut self) -> Weak<&C> {
		unimplemented!()
	}

	fn destroy_all_resources<C>(&mut self) -> Result<(), Error> {
		// for image in self.resource_lists.images {
		// 	unimplemented!()
		// }
		unimplemented!();

		Ok(())
	}

	fn destroy_resource<C>(&mut self, resource: &mut C) -> Result<(), Error> {
		//TODO_rust: should be part of the write_buffer() call instead?
		// self.device.destroy_buffer(resource.image_upload_buffer);

		self.device.destroy_image(resource.image_binding);

		self.device.destroy_image_view(resource.image_render_view);

		self.device.free_memory(resource.image_memory);
		unimplemented!();

		resource.mark_destroyed();

		Ok(())
	}
}