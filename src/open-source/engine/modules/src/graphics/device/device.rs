use std::fmt;
use super::BackendType;

/**
Represents a physical graphics device. This gets sent draw calls.
*/
pub trait Device {
	/**
	Returns the type of backend implementation
	used by this device.
	*/
	fn backend_type(&self) -> BackendType;
	/**
	Performs any per-frame cleanup the device may need
	to do.
	*/
	//May need to be &mut self?
	fn end_frame(&mut self);
}

impl fmt::Debug for Device {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Device {{ backend_type: {} }}", self.backend_type())
	}
}