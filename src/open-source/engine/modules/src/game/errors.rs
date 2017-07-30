/*!
Describes errors used in the game module.
*/
use std::error::Error;
use std::fmt;

/**
Errors for the game runner.
*/
#[derive(Debug)]
pub enum GameError {
	/**
	The current layout doesn't have a virtual key
	for the given character code.
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
			GameError::Unknown => { "unknown error" }
		}
	}
}
