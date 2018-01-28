use std::collections::HashMap;
use input::InputError;
use input::{CharacterCode, KeyCode, KeyfieldLayoutType};

pub struct KeyfieldLayout {
	pub layout_type: KeyfieldLayoutType,
	pub characters_to_keys: HashMap<CharacterCode, KeyCode>
}

impl KeyfieldLayout {
	pub fn has_character_code(&self, character: CharacterCode) -> bool {
		self.characters_to_keys.contains_key(&character)
	}

	pub fn get_key(&self, character: CharacterCode) -> Result<KeyCode, InputError> {
		if self.has_character_code(character) {
			return Ok(self.characters_to_keys[&character]);
		}
		Err(InputError::CharacterCodeUnsupported(self.layout_type, character))
	}
}