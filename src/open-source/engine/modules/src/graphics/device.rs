/*!
	The generic specification for a graphics wrapper.
*/
use std::fmt;
use std::sync::Arc;
use platform::{PlatformCode, current_platform};
use super::errors::BackendError;

/**
Specifies what graphics API the given device uses.
*/
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BackendType {
	DirectX,
	OpenGL,
	Vulkan,
	Other
}

impl fmt::Display for BackendType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

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

/**
Returns the graphics backends
available for the given platform.
*/
pub fn available_backends_for_platform(platform: PlatformCode) -> Vec<BackendType> {
	match platform {
		PlatformCode::Windows => {
			return vec!(
				BackendType::OpenGL//,
				//DirectX,	//not implemented yet
				//Vulkan	//not implemented yet
			)
		},
		PlatformCode::Linux => {
			return vec!(
				BackendType::OpenGL//,
				//Vulkan	//not implemented yet
			)
		},
		PlatformCode::MacOS => {
			return vec!(
				BackendType::OpenGL//,
				//Vulkan	//not implemented yet
			)
		},
		_ => { return vec!(); }
	}
}

pub fn available_backends() -> Vec<BackendType> {
	available_backends_for_platform(current_platform())
}

/**
Generic builder for graphics devices.
*/
#[derive(Debug)]
pub struct DeviceBuilder {
	//unimplemented!()
}

impl DeviceBuilder {
	pub fn new() -> DeviceBuilder {
		DeviceBuilder{}
	}

	/**
	Gets the default backend for the current platform.
	This should be the best performing backend
	that is also implemented for the platform.
	*/
	fn default_backend() -> Result<BackendType, BackendError> {
		//TEST:
		//	* Assert: backend returned is actually available
		//	according to device::available_backends() result
		let backends = available_backends();
		if backends.len() < 1 {
			return Err(BackendError::NoneAvailable);
		}

		Ok(backends[0])
	}

	/**
	Constructs a new graphics device context.
	
	Parameters:
	  * backend_type: The type of graphics backend to use with the physical device.
	*/
	pub fn build(&self, backend_type: BackendType) -> Result<Box<Device>, BackendError> {
		//Check that the backend is available on this platform.
		let backends = available_backends();
		let platform = current_platform();
		if !backends.contains(&backend_type) {
			return Err(BackendError::BackendUnavailable(backend_type, platform));
		}

		//Otherwise, get the backend builder for this backend type.
		unimplemented!();
		Ok(unimplemented!())
	}

	/**
	Constructs a new graphics device context,
	picking whatever backend is probably best for this platform.
	*/
	pub fn build_automatic_backend(&self) -> Result<Box<Device>, BackendError> {
		self.build(DeviceBuilder::default_backend()?)
	}
}