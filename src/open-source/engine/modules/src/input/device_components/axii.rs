use super::DeviceComponentList;

/**
	A control with a continuous range of values.
*/
pub struct Axii {
	pub num_axii: u32,
	pub min_values: Vec<f32>,
	pub max_values: Vec<f32>,
	pub current_values: Vec<f32>,
	pub normalized_values: Vec<f32>
}

impl DeviceComponentList for Axii {
	fn num_components(&self) -> u32 {
		self.num_axii
	}
}