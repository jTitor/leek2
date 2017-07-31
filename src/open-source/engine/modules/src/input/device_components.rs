/*!
	Defines elements of an input device.
*/
use std::collections::HashMap;
use super::errors::InputError;
use super::types::{CharacterCode, KeyCode, KeyfieldBlock, KeyfieldLayoutType};

pub trait DeviceComponentList {
	/**
		Returns the number of components of this type.
	*/
	fn num_components(&self) -> u32;
}

/**
	Any two-state control or set of two-state controls.
*/
pub trait TwoState {
}

/**
	A two-state control; it can only ever be pressed or released.
	Pressure-sensitive buttons are generally represented by the OS
	as an axis.
*/
pub struct Buttons {
	current_values: Vec<bool>
}

impl DeviceComponentList for Buttons {
	fn num_components(&self) -> u32 {
		self.current_values.len() as u32
	}
}

/**
	A control with more than two discrete states.
	The OS generally doesn't directly report these,
	since an axis basically does the same thing.
*/
pub struct MultiStateButtons {
	num_buttons: u32,
	min_values: Vec<i32>,
	max_values: Vec<i32>,
	current_values: Vec<i32>
}

impl DeviceComponentList for MultiStateButtons {
	fn num_components(&self) -> u32 {
		self.num_buttons
	}
}

/**
	A control with a continuous range of values.
*/
pub struct Axii {
	num_axii: u32,
	min_values: Vec<f32>,
	max_values: Vec<f32>,
	current_values: Vec<f32>,
	normalized_values: Vec<f32>
}

impl DeviceComponentList for Axii {
	fn num_components(&self) -> u32 {
		self.num_axii
	}
}

pub struct KeyfieldLayout {
	layout_type: KeyfieldLayoutType,
	characters_to_keys: HashMap<CharacterCode, KeyCode>
}

impl KeyfieldLayout {
	fn has_character_code(&self, character: CharacterCode) -> bool {
		self.characters_to_keys.contains_key(&character)
	}

	fn get_key(&self, character: CharacterCode) -> Result<KeyCode, InputError> {
		if self.has_character_code(character) {
			return Ok(self.characters_to_keys[&character]);
		}
		Err(InputError::CharacterCodeUnsupported(self.layout_type, character))
	}
}

pub struct SingleKeyfield {
	current_blocks: Vec<KeyfieldBlock>,
	prev_blocks: Vec<KeyfieldBlock>,
	layout: KeyfieldLayout,
}

impl SingleKeyfield {
	/**
		If true, `key` is down in the current frame.
	*/
	pub fn curr_key_down(&self, key: KeyCode) -> bool {
		unimplemented!()
	}

	/**
		If true, `key` is down in the previous frame.
	*/
	pub fn prev_key_down(&self, key: KeyCode) -> bool {
		unimplemented!()
	}

	//These could be moved to a generic signal on the device?
	//---
	/**
	If true, `key` is down in the current frame.
	*/
	pub fn key_down(&self, key: KeyCode) -> bool {
		self.curr_key_down(key)
	}

	/**
	If true, `key` is not down in the current frame.
	*/
	pub fn key_up(&self, key: KeyCode) -> bool {
		!self.key_down(key)
	}

	/**
	If true, `key` is down in the previous frame and in the current frame.
	*/
	pub fn key_held(&self, key: KeyCode) -> bool {
		self.prev_key_down(key) & self.curr_key_down(key)
	}

	/**
	If true, `key` transitioned from being up in the previous frame
	to being down in the current frame.
	*/
	pub fn key_pressed(&self, key: KeyCode) -> bool {
		!self.prev_key_down(key) & self.curr_key_down(key)
	}

	/**
	If true, `key` transitioned from being down in the previous frame
	to being up in the current frame.
	*/
	pub fn key_released(&self, key: KeyCode) -> bool {
		self.prev_key_down(key) & !self.curr_key_down(key)
	}

	/**
	If true, `key` is up in the previous frame and in the current frame.
	Not sure what this would describe.
	*/
	pub fn key_antiheld(&self, key: KeyCode) -> bool {
		!self.prev_key_down(key) & !self.curr_key_down(key)
	}
}

/**
	A control that represents a keyboard or similarly large and uniform cluster of keys.
*/
pub struct Keyfields {
	keyfields: Vec<SingleKeyfield>
}