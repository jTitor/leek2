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
			InputError::CharacterCodeUnsupported(_layout_type, _code) => "current layout doesn't have a key code for given character code"
		}
	}
}

impl fmt::Display for InputError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		#![allow()]
		let mut details: String = String::from(self.description());
		
		//Get a more elaborate description if possible.
		match *self {
			InputError::CharacterCodeUnsupported(layout_type, code) => {
				details = format!("current layout '{}' doesn't have a key code for given character code '{}'", layout_type, code);
			}
		}

		write!(f, "Input handler error: {}", details)
	}
}