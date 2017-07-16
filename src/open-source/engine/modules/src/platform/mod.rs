/*!
	Module definition for platform-dependent
	fields and operations.
*/

//Windows-only operations and wrappers.
mod windows;

//POSIX-only operations and wrappers.
mod posix;


//Definitions for platform identification here.
extern crate os_type;

/**
Represents the platform the library is compiled for.
*/
pub enum PlatformCode {
	Windows,
	Linux,
	MacOS,
	Unknown
}
//pub use os_type::OSType as PlatformCode;
//let os = os_type::current_platform();

/**
Returns the platform this library is compiled for.
*/
#[cfg(windows)]
pub fn current_platform() -> PlatformCode {
	PlatformCode::Windows
}

#[cfg(linux)]
pub fn current_platform() -> PlatformCode {
	PlatformCode::Linux
}

#[cfg(macos)]
pub fn current_platform() -> PlatformCode {
	PlatformCode::MacOS
}

#[cfg(not(windows, linux, macos))]
pub fn current_platform() -> PlatformCode {
	PlatformCode::Unknown
}