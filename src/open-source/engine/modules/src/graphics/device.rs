/*!
	The generic specification for a graphics wrapper.
*/
use platform::{PlatformCode, current_platform};

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
		PlatformCode::Windows {
			return vec!(
				OpenGL//,
				//DirectX,	//not implemented yet
				//Vulkan	//not implemented yet
			)
		},
		PlatformCode::Linux {
			return vec!(
				OpenGL//,
				//Vulkan	//not implemented yet
			)
		}
		PlatformCode::MacOS {
			return vec!(
				OpenGL//,
				//Vulkan	//not implemented yet
			)
		}
		_ -> return vec!();
	}
}

pub fn available_backends() -> Vec<BackendType> {
	available_backends(current_platform())
}

/**
Generic builder for graphics devices.
*/
#[derive(Debug)]
pub structDeviceBuilder {
	unimplemented!()
}

impl DeviceBuilder {
	pub fn new() -> DeviceBuilder {
		DeviceBuilder{}
	}

	/**
	Gets the default backend for the current platform.
	*/
	fn default_backend() -> Result<BackendType, Err> {
		//TEST:
		//	* Assert: backend returned is available
		//	according to device::available_backends() result
		match current_platform() {
			PlatformCode::Windows
		}
	}

	/**
	Constructs a new graphics device context.
	
	Parameters:
	  * type: The type of graphics backend to use with the physical device.
	*/
	pub fn build(&self, type: BackendType) -> Result<Device, Err> {
		//Check that the backend is available on this platform.
		unimplemented!();
		Ok(unimplemented!())
	}

	/**
	Constructs a new graphics device context,
	picking whatever backend is probably best for this platform.
	*/
	pub fn build_automatic_backend(&self) -> Result<Device, Err> {
		//Check the platform.
		//Pick whatever's suitable -
		//Windows probably should use DirectX,
		//POSIX should use OpenGL (since Vulkan's not fully supported yet).
		unimplemented!();
		Ok(unimplemented!())
	}
}