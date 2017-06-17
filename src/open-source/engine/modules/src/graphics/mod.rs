/*!
	Module definition for graphics operations.
*/

//The generic specification for a graphics wrapper.
pub mod device;
pub mod window;

pub use device::{Device, DeviceBuilder};
pub use device::BackendType;
pub use window::{Window, WindowBuilder};
pub use window::Visibility;
pub use window::EventType;