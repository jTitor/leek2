use super::DeviceComponentList;

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