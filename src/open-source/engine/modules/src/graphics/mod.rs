/*!
	Module definition for graphics operations.
*/

//The generic specification for a graphics wrapper.
pub mod device;
pub mod window;

pub use self::device::Device;
pub use self::device::{BackendType, BackendRequestType, available_backends, default_backend};

pub use self::window::Window;
pub use self::window::Visibility;
pub use self::window::EventType;

pub mod errors;
pub use self::errors::BackendError;
pub use self::window::{WindowError, WindowBuilderError};

pub mod factory;
pub use self::factory::{GraphicsFactory, GraphicsPayload};

pub mod diagnostic;

pub mod scene;