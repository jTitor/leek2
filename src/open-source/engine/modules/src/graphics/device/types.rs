/**
Specifies what graphics API the given device uses.
*/
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BackendType {
	DirectX,
	OpenGL,
	Vulkan,
	Other
}

impl fmt::Display for BackendType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}