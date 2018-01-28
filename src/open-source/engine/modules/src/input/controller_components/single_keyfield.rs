use input::{KeyCode, KeyfieldBlock, KeyfieldLayout};

pub struct SingleKeyfield {
	pub current_blocks: Vec<KeyfieldBlock>,
	pub prev_blocks: Vec<KeyfieldBlock>,
	pub layout: KeyfieldLayout,
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