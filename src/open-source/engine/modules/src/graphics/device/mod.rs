/*!
	The generic specification for a graphics wrapper.
*/
pub mod device;
pub mod types;
pub mod internal;

pub use self::device::Device;
pub use self::types::{BackendType, BackendRequestType};

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

pub fn default_backend_for_platform(platform: PlatformCode) -> BackendType {
	match platform {
		PlatformCode::Windows => {
			return BackendType::OpenGL; //DirectX not implemented yet
		},
		PlatformCode::Linux |
		PlatformCode::MacOS => {
			return BackendType::OpenGL;
		},
		_ => { return BackendType::Other; }
	}
}

pub fn default_backend() -> BackendType {
	default_backend_for_platform(current_platform())
}