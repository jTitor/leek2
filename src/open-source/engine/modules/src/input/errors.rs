/*!
Describes errors used in the input module.
*/
use std::error::Error;
use super::types::{CharacterCode, KeyfieldLayoutType};

/**
Errors for backend implementations.
*/
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
				format!("layout {} doesn't have a key code for character code {}", layout_type, code)
			}
		}
	}
}
