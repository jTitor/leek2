/*!
Describes errors used in the window module.
*/
use std::error::Error;
use std::fmt;

/**
Errors for the window builder.
*/
#[derive(Debug)]
pub enum WindowBuilderError {
	Unknown
}

impl Error for WindowBuilderError {
	fn description(&self) -> &str {
		match *self {
			WindowBuilderError::Unknown => "unknown error",
		}
	}
}

impl fmt::Display for WindowBuilderError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Window builder error: {}", self.description())
	}
}

#[derive(Debug)]
pub enum WindowError {
	BackendOperationFailed,
	Unknown
}

impl Error for WindowError {
	fn description(&self) -> &str {
		match *self {
			WindowError::BackendOperationFailed => "graphics backend operation failed",
			WindowError::Unknown => "unknown error",
		}
	}
}

impl fmt::Display for WindowError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Window error: {}", self.description())
	}
}
