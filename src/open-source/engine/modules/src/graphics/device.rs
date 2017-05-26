/*!
	The generic specification for a graphics wrapper.
*/

/**
Specifies what graphics API the given device uses.
*/
pub enum BackendType {
	DirectX,
	OpenGL,
	Vulkan,
	Other
}

/**
Represents a physical graphics device. This gets sent draw calls.
*/
pub trait Device {
	fn backend_type(&self) -> BackendType;
}