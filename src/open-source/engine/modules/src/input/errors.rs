/*!
Describes errors used in the input module.
*/
use std::error::Error;
use std::fmt;
use super::types::{CharacterCode, KeyfieldLayoutType};

/**
Errors for backend implementations.
*/
#[derive(Debug)]
pub enum InputError {
	/**
	The current layout doesn't have a virtual key
	for the given character code.
	*/
	CharacterCodeUnsupported(KeyfieldLayoutType, CharacterCode)
}

impl Error for InputError {
	fn description(&self) -> &str {
		match *self {
			InputError::CharacterCodeUnsupported(layout_type, code) => {
				format!("layout {} doesn't have a key code for character code {}", layout_type, code).as_str()
			}
		}
	}
}

impl fmt::Display for InputError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Input handler error: {}", self.description())
	}
}