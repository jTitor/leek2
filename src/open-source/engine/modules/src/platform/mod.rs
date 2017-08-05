/*!
	Module definition for platform-dependent
	fields and operations.
*/
use std::fmt;

//Windows-only operations and wrappers.
mod windows;

//POSIX-only operations and wrappers.
mod posix;


//Definitions for platform identification here.
//extern crate os_type;

/**
Represents the platform the library is compiled for.
*/
#[derive(Debug, PartialEq)]
pub enum PlatformCode {
	Windows,
	Linux,
	MacOS,
	Unknown
}

impl fmt::Display for PlatformCode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

//pub use os_type::OSType as PlatformCode;
//let os = os_type::current_platform();

/**
Returns the platform this library is compiled for.
*/
#[cfg(windows)]
fn get_current_platform() -> PlatformCode {
	PlatformCode::Windows
}

#[cfg(linux)]
fn get_current_platform() -> PlatformCode {
	PlatformCode::Linux
}

#[cfg(target_os = "macos")]
fn get_current_platform() -> PlatformCode {
	PlatformCode::MacOS
}

#[cfg(not(any(target_os = "macos", windows, linux)))]
fn get_current_platform() -> PlatformCode {
	PlatformCode::Unknown
}

pub fn current_platform() -> PlatformCode {
	get_current_platform()
}