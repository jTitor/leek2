/*!
 * Contains fields and structs used to build
 * command queues and buffers for a device.
 */
use gfx_hal as hal;
use std::rc::Rc;
use failure::Error;

pub struct DeviceInfo<B: hal::Backend> {
	/**
	 * TODO
	 */
	surface_format: hal::Format,
	/**
	 * TODO
	 */
	memory_types: Vec<hal::MemoryType>,
	/**
	 * TODO
	 */
	limits: hal::Limits
}

type DefaultSurfaceFormat = hal::Format::Rgba8Srgb;

impl<B: hal::Backend> DeviceInfo<B> {
	pub fn from_backend(adapter: Rc<hal::Adapter<B>>, surface: Rc<&hal::Surface>) -> Result<DeviceInfo, Error> {
		//Setup the device's surface format:
		let surface_format = surface
			//Get the first valid one if possible.
			.capabilities_and_formats(&adapter.physical_device)
			.1
			.map_or(
				//If .1 is None, pick the default format
				//instead
				DefaultSurfaceFormat,
				//Otherwise, pull the format's
				//color format.
				|formats| {
					formats
						.into_iter()
						.find(|format| {
							format.base_format().1 == hal::ChannelType::Srgb
						})?
				}
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