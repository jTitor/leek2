use super::DeviceComponentList;

/**
	A control with more than two discrete states.
	The OS generally doesn't directly report these,
	since an axis basically does the same thing.
*/
pub struct MultiStateButtons {
	pub num_buttons: u32,
	pub min_values: Vec<i32>,
	pub max_values: Vec<i32>,
	pub current_values: Vec<i32>
}

impl DeviceComponentList for MultiStateButtons {
	fn num_components(&self) -> u32 {
		self.num_buttons
	}
}