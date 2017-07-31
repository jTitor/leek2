/*!
Describes errors used in the graphics module.
*/
use std::error::Error;
use std::fmt;
use super::device::BackendType;
use platform::PlatformCode;

/**
Errors for backend implementations.
*/
#[derive(Debug)]
pub enum BackendError {
	///The current platform doesn't support
	///any backends.
	NoneAvailable,
	///The requested backend is unavailable
	///for the given platform.
	BackendUnavailable(BackendType, PlatformCode)
}

impl fmt::Display for BackendError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Device backend error: {}", self.description())
	}
}

impl Error for BackendError {
	fn description(&self) -> &str {
		match *self {
			BackendError::NoneAvailable => "no suitable backend available",
			BackendError::BackendUnavailable(backend_type, platform_code) => {
				format!("backend {} isn't available on platform {}", backend_type, platform_code).as_str()
			}
		}
	}
}
