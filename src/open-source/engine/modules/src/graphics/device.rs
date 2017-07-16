/*!
	The generic specification for a graphics wrapper.
*/
use platform::{PlatformCode, current_platform};
use graphics::BackendError;

/**
Specifies what graphics API the given device uses.
*/
pub enum BackendType {
	DirectX,
	OpenGL,
	Vulkan,
	Other
}

/**
Represents a physical graphics device. This gets sent draw calls.
*/
pub trait Device {
	/**
	Returns the type of backend implementation
	used by this device.
	*/
	pub fn backend_type(&self) -> BackendType;
	/**
	Performs any per-frame cleanup the device may need
	to do.
	*/
	//May need to be &mut self?
	pub fn end_frame(&mut self);
}

/**
Returns the graphics backends
available for the given platform.
*/
pub fn available_backends(platform: PlatformCode) -> Vec<BackendType> {
	match platform {
		PlatformCode::Windows => {
			return vec!(
				OpenGL//,
				//DirectX,	//not implemented yet
				//Vulkan	//not implemented yet
			)
		},
		PlatformCode::Linux => {
			return vec!(
				OpenGL//,
				//Vulkan	//not implemented yet
			)
		},
		PlatformCode::MacOS => {
			return vec!(
				OpenGL//,
				//Vulkan	//not implemented yet
			)
		},
		_ => return vec!();
	}
}

pub fn available_backends() -> Vec<BackendType> {
	available_backends(current_platform())
}

/**
Generic builder for graphics devices.
*/
#[derive(Debug)]
pub struct DeviceBuilder {
	unimplemented!()
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
	fn default_backend() -> Result<BackendType, Err> {
		//TEST:
		//	* Assert: backend returned is actually available
		//	according to device::available_backends() result
		let backends = available_backends();
		if backends.len() < 1 {
			return BackendError::NoneAvailable;
		}

		Ok(backends[0])
	}

	/**
	Constructs a new graphics device context.
	
	Parameters:
	  * type: The type of graphics backend to use with the physical device.
	*/
	pub fn build(&self, type: BackendType) -> Result<Device, Err> {
		//Check that the backend is available on this platform.
		let backends = available_backends();
		if !backends.contains(&type) {
			return Err(unimplemented!());
		}

		//Otherwise, get the backend builder for this backend type.
		unimplemented!();
		Ok(unimplemented!())
	}

	/**
	Constructs a new graphics device context,
	picking whatever backend is probably best for this platform.
	*/
	pub fn build_automatic_backend(&self) -> Result<Device, Err> {
		self.build(default_backend())
	}
}