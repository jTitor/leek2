/*!
 * Builder struct for a MemoryBuffer.
 */
use graphics::device::internal::pipeline::{DeviceController, MemoryBuffer};

use failure::Error;
use gfx_hal as hal;
use hal::memory as m;

pub enum BufferType {
	Vertex,
	/** Any data that isn't per-vertex. */
	Data
}

impl From<BufferType> for buffer::Usage {
	fn from(x: BufferType) -> Self {
		match x {
			BufferType::Vertex => buffer::Usage::VERTEX,
			BufferType::Data => buffer::Usage::TRANSFER_SRC
		}
	}
}

/**
 * Specifies where data for the buffer
 * can be uploaded from.
 */
pub enum BufferUploadType {
	FromCPU,
	FromGPU
}

impl From<BufferUploadType> for m::Properties {
	fn from(x: BufferUploadType) -> Self {
		match x {
			BufferUploadType::FromCPU => m::Properties::CPU_VISIBLE,
			BufferUploadType::FromGPU => m::Properties::DEVICE_LOCAL
		}
	}
}

//TODO: Make this a request sent to
//the DeviceController
pub struct BufferBuilder {
	buffer_type: BufferType,
	upload_type: BufferUploadType,
	size_bytes: usize
}

pub impl BufferBuilder {
	//TODO: DeviceController should handle this
	//instead
	pub fn build(&self) -> Result<MemoryBuffer, Error> {
		//TODO: everything up to "END TODO"
		//should be genericized,
		//as images ues get_image_requirements
		//instead
		let cast_buffer_type = Into<buffer::Usage>::into(self.buffer_type);
		let device_upgrade = self.device.upgrade();

		if let Some(device) = device_upgrade {
			let buffer_unbound = device.create_buffer(self.size_bytes, cast_buffer_type)?;
			let buffer_req = device.get_buffer_requirements(&buffer_unbound);

			let cast_upload_property = Into<m::Properties>::into(self.upload_type);
			//END TODO
			let upload_type = memory_types
				.iter()
				.enumerate()
				.position(|(id, mem_type)| {
					buffer_req.type_mask & (1 << id) != 0 &&
					mem_type.properties.contains(cast_upload_property)
				})
				.unwrap()
				.into();

			let buffer_memory = device.allocate_memory(upload_type, buffer_req.size)?
			let buffer_object = device.bind_buffer_memory(&buffer_memory, 0, buffer_unbound)?

			Ok(MemoryBuffer {
				device: self.device.clone(),
				buffer: buffer_unbound,
				buffer_memory: buffer_memory,
				buffer_binding: buffer_object,
				buffer_len: buffer_req.size,
				resources_destroyed: false;
			})
		}

		Error; //TODO: specify error
	}
}