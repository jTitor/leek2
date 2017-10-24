/*!
Describes errors used in the game module.
*/
use std::error::Error;
use std::fmt;
use graphics::{BackendError};
use graphics::window::WindowBuilderError;

/**
Errors for the game runner.
*/
#[derive(Debug)]
pub enum GameError {
	/**
	One of the game's devices had a fatal error.
	*/
	DeviceError { cause: BackendError },
	WindowBuildFailed { cause: WindowBuilderError },
	/**
	The game can't perform the requested command
	under the current platform.
	*/
	PlatformUnsupported,
	/**
	 The game can perform the given command,
	 but the parameters are out of bounds in some way.
	 */
	InvalidRequest,
	/**
	Error unknown.
	*/
	Unknown
}

impl fmt::Display for GameError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Game error: {}", self.description())
	}
}

impl Error for GameError {
	fn description(&self) -> &str {
		match *self {
			GameError::DeviceError{ref cause} => { "a device had a fatal error" },
			GameError::WindowBuildFailed{ref cause} => { "a window could not be built" },
			GameError::PlatformUnsupported => { "this platform doesn't support the requested command" },
			GameError::InvalidRequest => { "a window could not be built" },
			_ => { "unknown error" }
		}
	}

	fn cause(&self) -> Option<&Error> {
		match *self {
			GameError::DeviceError{ref cause} => Some(cause),
			GameError::WindowBuildFailed{ref cause} => Some(cause),
			_ => None
		}
	}
}

impl From<BackendError> for GameError {
	fn from(error: BackendError) -> Self {
		match error {
			_ => GameError::DeviceError{ cause: error }
		}
	}
}

impl From<WindowBuilderError> for GameError {
	fn from(error: WindowBuilderError) -> Self {
		match error {
			_ => GameError::WindowBuildFailed{ cause: error }
		}
	}
}