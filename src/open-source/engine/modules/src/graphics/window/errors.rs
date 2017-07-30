/*!
Describes errors used in the window module.
*/
use std::error::Error;

/**
Errors for the window builder.
*/
pub enum WindowBuilderError {
	Unknown
}

impl Error for WindowBuilderError {
	fn description(&self) -> &str {
		match *self {
			WindowBuilderError => "unknown error",
		}
	}
}
