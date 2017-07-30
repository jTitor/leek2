/*!
	Module definition for graphics operations.
*/

//The generic specification for a graphics wrapper.
pub mod device;
pub mod window;

pub use self::device::{Device, DeviceBuilder};
pub use self::device::{BackendType, available_backends};
pub use self::window::{Window, WindowBuilder};
pub use self::window::Visibility;
pub use self::window::EventType;

pub mod errors;
pub use self::errors::BackendError;