use std::error::Error;
use std::fmt;

use remotery::error::RemoteryError;

#[derive(Debug, Copy, Clone)]
pub struct ProfilerError {
}

impl Error for ProfilerError {
	fn description(&self) -> &str {
		"unknown error in profiler backend"
	}

	// fn cause(&self) -> Option<&Error> {
	// 	//TODO: Not sure how we're going to go about error chaining. For now keep this in case
	// 	//we do want to do it.
	// 	match *self {
	// 		GameError::DeviceError{ref cause} => Some(cause),
	// 		GameError::WindowBuildFailed{ref cause} => Some(cause),
	// 		_ => None
	// 	}
	// }
}

impl fmt::Display for ProfilerError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut details: String = String::from(self.description());

		write!(f, "Profiler error: {}", details)
	}
}

impl From<RemoteryError> for ProfilerError {
	fn from(error: RemoteryError) -> Self {
		match error {
			_ => ProfilerError{}
		}
	}
}