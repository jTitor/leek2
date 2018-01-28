/*!
	Module definition for input methods.
*/
pub mod errors;
pub use self::errors::InputError;
pub mod types;
pub use self::types::{CharacterCode, KeyCode, KeyfieldBlock, KeyfieldLayoutType, SignalCode};

//Defines elements of an input device.
pub mod controller;
pub use self::controller::{Controller, ControllerType};
pub mod controller_components;
pub use self::controller_components::{Axii, Buttons, SingleKeyfield, KeyfieldLayout};

//Defines filtering operations on input devices.
pub mod filtering;