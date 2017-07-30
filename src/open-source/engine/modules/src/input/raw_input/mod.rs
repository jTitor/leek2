/*!
Handles raw input for different platforms.
*/

pub mod interface;

pub use interface::RawInputHandler;

//An implementation for each platform is needed.
/*
//In particular, Windows needs overrides for raw input.
//tomaka/winit (https://github.com/tomaka/winit/tree/master/src/platform/windows)
//uses raw input for the mouse, but not keyboards or gamepads
//and it doesn't enumerate RI devices.
//tomaka/winit also doesn't have a generic device event apparently,
//the WM_INPUT handler is for the first mouse's movements only.
//The Event enum can definitely handle arbitrary devices and controls though.
//RI registration can be done outside of winit since
//that's done by winAPI calls.
//This means you need to fork winit, THEN glutin to use your forked winit.
pub mod windows;
pub mod macos;
pub mod linux;
*/