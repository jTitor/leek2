use super::super::errors::BackendError;
use super::types::BackendType;
use super::device::Device;
use platform::current_platform;
use super::available_backends;

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