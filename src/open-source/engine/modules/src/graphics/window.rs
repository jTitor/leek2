/*!
	Generic specification for a window.
*/

use super::super::math::linear_algebra::vec2::Vec2;
use super::device::Device;

/**
Specifies the window's state on screen.
*/
pub enum Visibility {
	Closed,
	Normal,
	Minimized,
	Maximized,
	Fullscreen
}

/**
Generic specification for a window.
Through this you can modify window parameters.
*/
pub trait Window {
	/**
	Returns the window's visibility state.
	*/
	fn visibility(&self) -> Visibility;

	/**
	Returns the window's position on screen.
	Is invalid if Window.is_open() == false.
	*/
	fn position(&self) -> Vec2;

	/**
	Returns the window's dimensions on screen.
	Is invalid if Window.is_open() == false.
	*/
	fn dimensions(&self) -> Vec2;

	/**
	If true, the window exists in
	the windowing manager.
	It's not necessarily visible.
	*/
	fn is_open(&self) -> bool;

	/**
	Opens the window once built.
	*/
	fn open(&self) -> Result<>;

	/**
	Closes the window.
	If the window hasn't been opened, does nothing.
	*/
	fn close(&self) -> Result<>;
}

/**
Generic specification for window builders.
*/
pub trait WindowBuilder {
	fn build(&self, graphics: Device) -> Result<Window, Error>;
}