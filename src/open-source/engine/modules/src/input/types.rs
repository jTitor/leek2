/**
	Represents the keys on a keyboard.
*/
pub type KeyfieldBlock u128;
/**
	Represents the key code presented by an OS event;
	this is what the OS says was pressed.
*/
pub type CharacterCode u32;
/**
	Represents a key or button within the engine.
*/
pub type KeyCode u32;
/**
	Represents a key, button, or axis within the engine.
*/
pub type SignalCode u32;

/**
Specifies a mapping of characters and OS-specific
virtual keys to engine-standardized virtual keys.
*/
pub enum KeyfieldLayoutType {
	QWERTY
}