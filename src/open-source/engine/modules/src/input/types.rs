use std::fmt;

/**
	Represents the keys on a keyboard.
*/
pub type KeyfieldBlock = u64;
/**
	Represents the key code presented by an OS event;
	this is what the OS says was pressed.
*/
pub type CharacterCode = u32;
/**
	Represents a key or button within the engine.
*/
pub type KeyCode = u32;
/**
	Represents a key, button, or axis within the engine.
*/
pub type SignalCode = u32;

/**
Specifies a mapping of characters and OS-specific
virtual keys to engine-standardized virtual keys.
*/
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeyfieldLayoutType {
	QWERTY
}

impl fmt::Display for KeyfieldLayoutType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}