/*!
 * Defines the DeviceInfo struct.
 */
use gfx_hal as hal;
use gfx_hal::PhysicalDevice;
use gfx_hal::format as f;
use gfx_hal::format::AsFormat;
use std::rc::Rc;
use failure::Error;

/**
 * Contains fields and structs used to build
 * command queues and buffers for a device.
 */
pub struct DeviceInfo {
	/**
	 * TODO
	 */
	surface_format: f::Format,
	/**
	 * TODO
	 */
	memory_types: Vec<hal::MemoryType>,
	/**
	 * TODO
	 */
	limits: hal::Limits
}

const DEFAULT_SURFACE_FORMAT: f::Format = f::Rgba8Srgb::SELF;

impl DeviceInfo {
	pub fn from_backend<B>(adapter: Rc<hal::Adapter<B>>, surface: Rc<&hal::Surface<B>>) -> Result<DeviceInfo, Error> where B: hal::Backend {
		//Setup the device's surface format:
		let surface_format = surface
			//Get the first valid one if possible.
			.capabilities_and_formats(&adapter.physical_device)
			.1
			.map_or(
				//If .1 is None, pick the default format
				//instead
				DEFAULT_SURFACE_FORMAT,
				//Otherwise, pull the format's
				//color format.
				|formats| {
					formats
						.into_iter()
						.find(|format| {
							format.base_format().1 == f::ChannelType::Srgb
						})
				}.unwrap_or_else(|| { return Error; })
			);

		//and memory/physical properties.
		let memory_types = adapter
			.physical_device
			.memory_properties()
			.memory_types;

		let limits = adapter
			.physical_device
			.get_limits();

		Ok(DeviceInfo {
			surface_format: surface_format,
			memory_types: memory_types,
			limits: limits
		})
	}
}