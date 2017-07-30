/*!
Describes errors used in the game module.
*/
use std::error::Error;
/**
Errors for the game runner.
*/
pub enum GameError {
	/**
	The current layout doesn't have a virtual key
	for the given character code.
	*/
	Unknown
}

impl Error for InputError {
	fn description(&self) -> &str {
		match *self {
			_ => { "unknown error" }
		}
	}
}
