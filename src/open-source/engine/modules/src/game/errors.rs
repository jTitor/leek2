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
			GameError::DeviceError{cause} => { "a device had a fatal error" },
			GameError::WindowBuildFailed{cause} => { "a window could not be built" },
			_ => { "unknown error" }
		}
	}

	fn cause(&self) -> Option<&Error> {
		match *self {
			GameError::DeviceError{cause} => Some(&cause),
			GameError::WindowBuildFailed{cause} => Some(&cause),
			GameError::Unknown => None
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