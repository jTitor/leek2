/*!
Describes errors used in the graphics module.
*/
use std::error::Error;

/**
Errors for backend implementations.
*/
pub enum BackendError {
	///The current platform doesn't support
	///any backends.
	NoneAvailable
}

impl Error for BackendError {
	fn description(&self) -> &str {
		match *self {
			BackendError::NoneAvailable => "no suitable backend available"
		}
	}
}
