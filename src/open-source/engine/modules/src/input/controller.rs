/*!
	Defines input devices.
*/
use super::controller_components::{Axii, Buttons, SingleKeyfield};

pub enum ControllerType {
	Mouse,		//Generally has an X and Y axis, and at least one button.
	Keyboard,	//Keyboards have many buttons, so their state is usually represented as a bitfield.
	HID			//All other input devices, such as joysticks, gamepads or drawing tablets.
}

/**
An object that holds
multiple individual inputs.
Generally this represents a physical
input device.
*/
pub struct Controller {
	pub controller_type: ControllerType,
	pub axii: Axii,
	pub buttons: Buttons,
	pub keys: SingleKeyfield
}