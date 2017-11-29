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
		let mut details: String = String::from(self.description());

		if let BackendError::BackendUnavailable(backend_type, platform_code) = *self {
			details = format!("backend '{}' isn't available on current platform '{}'", backend_type, platform_code);
		}

		write!(f, "Device backend error: {}", details)
	}
}

impl Error for BackendError {
	fn description(&self) -> &str {
		match *self {
			BackendError::NoneAvailable => "no suitable backend available",
			BackendError::BackendUnavailable(_backend_type, _platform_code) => "backend isn't available on current platform"
		}
	}
}