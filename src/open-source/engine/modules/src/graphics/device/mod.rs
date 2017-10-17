/*!
	The generic specification for a graphics wrapper.
*/
mod device;
mod types;

pub use self::device::Device;
pub use self::types::BackendType;

use platform::{PlatformCode, current_platform};

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