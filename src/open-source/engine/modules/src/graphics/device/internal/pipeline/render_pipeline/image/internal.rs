/*!
 * Internal traits and structs used by the Image
 * struct.
 */
use super::Image;

use failure::Error;
use gfx_hal as hal;

/**
 * Trait for initialization methods of Image.
 */
pub trait ImageInit {
	//TEMP METHOD: Loads the file into memory
	//and retrieves all the info needed
	//to create a buffer to upload it to.
	fn load_file(file_name: String) -> Result<(), Error>;

	//TEMP METHOD: Creates the image buffers used to
	//temporarily store the image before
	//binding to a render target.
	fn create_upload_buffer() -> Result<(), Error>;

	fn copy_image_to_upload_buffer() -> Result<(), Error>;

	fn create_image_object() -> Result<(), Error>;

	fn copy_upload_buffer_to_image_object() -> Result<(), Error>
}

impl<B: hal::Backend> ImageInit for Image<B> {
	fn load_file(file_name: String) -> Result<(), Error> {
		//TODO: make appropriate fs call
		let img_data = [0u8];//include_bytes!(file_name.as_str());

		let img = image::load(Cursor::new(&img_data[..]), image::PNG).unwrap().to_rgba();
		let (width, height) = img.dimensions();
		//Setup image specification
		let kind = i::Kind::D2(width as i::Size, height as i::Size, i::AaMode::Single);
		//The image is unpacked as 32-bit RGBA, so
		//each pixel is 4 bytes...
		let image_stride = 4usize;
		//...but we may need to ensure the memory allocated
		//is aligned too, so the final allocation may be larger than
		//needed.
		//Ask the limit info for the smallest alignment requirement.
		let row_alignment_mask = limits.min_buffer_copy_pitch_alignment as u32 - 1;
		//Now round the data length of a row to that alignment.
		let row_pitch = (width * image_stride as u32 + row_alignment_mask) & !row_alignment_mask;
		//That gives us the actual buffer size.
		let upload_size = (height * row_pitch) as u64;

		unimplemented!();
		Ok(())
	}

	fn create_upload_buffer() -> Result<(), Error> {
		//Set up the raw buffer object here...
		let image_buffer_unbound = device.create_buffer(upload_size, buffer::Usage::TRANSFER_SRC).unwrap();
		//The requirement that we be able to read from CPU
		//will let us allocate the right GPU memory here.
		let image_mem_reqs = device.get_buffer_requirements(&image_buffer_unbound);
		let image_upload_memory = device.allocate_memory(upload_type, image_mem_reqs.size).unwrap();
		//Now that we have the memory, bind it
		//to the buffer object.
		let image_upload_buffer = device.bind_buffer_memory(&image_upload_memory, 0, image_buffer_unbound).unwrap();

		//(we'll still need to return all of this,
		//since each object must be destroyed by
		//the device to properly release the resources.)
		unimplemented!();
		Ok(())
	}

	fn copy_image_to_upload_buffer() -> Result<(), Error> {
		let mut data = device
			.acquire_mapping_writer::<u8>(&image_upload_memory, 0..upload_size)?;

		for y in 0 .. height as usize {
			//Get the raw row bytes from source.
			let row = &(*img)[y*(width as usize)*image_stride .. (y+1)*(width as usize)*image_stride];
			//The destination may have a different width,
			let dest_base = y * row_pitch as usize;
			//but the remainder is just padding for
			//memory alignment.
			data[dest_base .. dest_base + row.len()].copy_from_slice(row);
		}

		device.release_mapping_writer(data);
		
		Ok(())
	}

	fn create_image_object() -> Result<(), Error> {
		//TODO: everything from here to "END TODO"
		//should be handled by an ImageBufferBuilder.
		//
		//The sample code sets Usage::TRANSFER_DST
		//so the image can be uploaded from CPU to GPU,
		//and Usage::SAMPLED so the sampler can get at it
		let image_unbound = device.create_image(kind, 1, ColorFormat::SELF, i::Usage::TRANSFER_DST | i::Usage::SAMPLED).unwrap(); // TODO: usage
		let image_req = device.get_image_requirements(&image_unbound);

		let device_type = memory_types
			.iter()
			.enumerate()
			.position(|(id, memory_type)| {
				image_req.type_mask & (1 << id) != 0 &&
				memory_type.properties.contains(m::Properties::DEVICE_LOCAL)
			})
			.unwrap()
			.into();
		let image_memory = device.allocate_memory(device_type, image_req.size).unwrap();
		//END TODO

		//This is the part unique to Image;
		//in gfx_hal an image object
		//is distinct from a buffer object
		//and also has an image view that
		//samplers connect to.
		let image_logo = device.bind_image_memory(&image_memory, 0, image_unbound).unwrap();
		let image_srv = device.create_image_view(&image_logo, ColorFormat::SELF, Swizzle::NO, COLOR_RANGE.clone()).unwrap();

		Ok(())
	}

	fn copy_upload_buffer_to_image_object() -> Result<(), Error> {
		//This is the command queue
		//call, but should be submitted to a Device.
		//The Image struct doesn't handle this itself.
		let submit = {
			let mut cmd_buffer = command_pool.acquire_command_buffer(false);

			let image_barrier = m::Barrier::Image {
				states: (i::Access::empty(), i::ImageLayout::Undefined) ..
						(i::Access::TRANSFER_WRITE, i::ImageLayout::TransferDstOptimal),
				target: &image_logo,
				range: COLOR_RANGE.clone(),
			};
			cmd_buffer.pipeline_barrier(PipelineStage::TOP_OF_PIPE .. PipelineStage::TRANSFER, &[image_barrier]);

			cmd_buffer.copy_buffer_to_image(
				&image_upload_buffer,
				&image_logo,
				i::ImageLayout::TransferDstOptimal,
				&[command::BufferImageCopy {
					buffer_offset: 0,
					buffer_width: row_pitch / (image_stride as u32),
					buffer_height: height as u32,
					image_layers: i::SubresourceLayers {
						aspects: f::AspectFlags::COLOR,
						level: 0,
						layers: 0 .. 1,
					},
					image_offset: command::Offset { x: 0, y: 0, z: 0 },
					image_extent: d::Extent { width, height, depth: 1 },
				}]);

			let image_barrier = m::Barrier::Image {
				states: (i::Access::TRANSFER_WRITE, i::ImageLayout::TransferDstOptimal) ..
						(i::Access::SHADER_READ, i::ImageLayout::ShaderReadOnlyOptimal),
				target: &image_logo,
				range: COLOR_RANGE.clone(),
			};
			cmd_buffer.pipeline_barrier(PipelineStage::TRANSFER .. PipelineStage::FRAGMENT_SHADER, &[image_barrier]);

			cmd_buffer.finish()
		};

		//The submission call for this
		//upload is different from a draw scene call:
		// let submission = Submission::new()
		// 	.submit(Some(submit));
		// self.main_queue.submit(submission, Some(&mut self.frame_fence));

		// self.device.wait_for_fence(&self.frame_fence, self.frame_wait_timeout_ms);

		//The draw scene call is instead:
		// let submission = Submission::new()
		// 	.wait_on(&[(&self.frame_semaphore, PipelineStage::BOTTOM_OF_PIPE)])
		// 	.submit(Some(submit));
		// self.main_queue.submit(submission, Some(&mut self.frame_fence));

		Ok(())
	}
}