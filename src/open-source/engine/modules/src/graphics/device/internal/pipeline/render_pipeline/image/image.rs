/*!
 * Contains the memory and buffer binding
 * representing an image.
 */
use graphics::device::internal::pipeline::DeviceResource;
use super::ImageInit;

use std::rc::Weak;

use failure::Error;
use gfx_hal as hal;
use gfx_hal::Device;

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
	pub fn mark_destroyed(&mut self) {
		debug_assert!(!self.resources_destroyed,
			"Image already marked as destroyed");

		self.resources_destroyed = true;
	}

	fn from_file(file_name: String) -> Result<Image<B>, Error> {
		//TODO: This part should be split out into
		//a separate struct - a RawImage?
		//Then you can have multiple providers or
		//a dummy provider for unit tests
		Self::load_file(file_name)?;

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
		let to_image_object_submission =  Self::copy_upload_buffer_to_image_object()?;
		//device.???(to_image_object_submission);
		
		unimplemented!();
	}
}

impl<B: hal::Backend> Drop for Image<B> {
	fn drop(&mut self) {
		debug_assert!(self.resources_destroyed, "Image went out of scope without having its memory destroyed");
	}
}

impl<B: hal::Backend> DeviceResource<B> for Image<B> {
	fn get_resource(device: &mut B::Device) -> Weak<&Self> {
		unimplemented!()
	}

	fn destroy_all_resources(device: &mut B::Device, resource_list: &Vec<Self>) -> Result<(), Error> {
		// for image in self.resource_lists.images {
		// 	unimplemented!()
		// }
		unimplemented!();

		Ok(())
	}

	fn destroy_resource(device: &mut B::Device, resource: &mut Self) -> Result<(), Error> {
		//TODO_rust: should be part of the write_buffer() call instead?
		// self.device.destroy_buffer(resource.image_upload_buffer);

		device.destroy_image(resource.image_binding);

		device.destroy_image_view(resource.image_render_view);

		device.free_memory(resource.image_memory);
		unimplemented!();

		resource.mark_destroyed();

		Ok(())
	}
}