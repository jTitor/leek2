/*!
Generic description of a raw input handler.
*/

use super::super::graphics::Window;
use super::super::device_components::{Device, DeviceType, DeviceIdentifier};

/**
Handles raw input events,
and gives links to input devices.
*/
pub trait RawInputHandler {
	/**
	Links handler to a window so it can recieve and
	send raw input events.
	*/
	pub fn connect(&self, window: &Window) -> Result<(), unimplemented!()>;

	/**
	Disconnects a handler from a window.
	Automatically called when the handler is dropped,
	but can be manually called too.
	If the handler isn't connected to a window,
	this does nothing.
	*/
	pub fn disconnect(&self);

	/**
	Gets the number of devices of the given type
	connected to this system.
	*/
	pub fn num_devices(&self, type: DeviceType) -> u64;

	/**
	Gets the device corresponding to the given device identifier.
	If the identifier is invalid, returns an Error.
	*/
	pub fn get_device(&self, device: DeviceIdentifier) -> Result<Device, ()>;
}