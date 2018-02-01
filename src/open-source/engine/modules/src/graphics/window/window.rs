/*!
	Generic specification for a window.
	Windows are separate from graphics devices,
	but depend on them to perform rendering;
	it's possible to have multiple windows for the
	same application, but the current Game
	implementation doesn't make use of this.
*/

use std::fmt;
use math::Vec2;
use graphics::{EventType, Visibility};
use input::Controller;
use super::WindowError;

/**
A dummy callback that does nothing.
*/
fn default_callback(_: EventType) {}

/**
Generic specification for a window.
Through this you can modify window parameters.
*/
pub trait Window {
	/**
	Returns the window's title.
	*/
	fn title(&self) -> &str;

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
	It's not necessarily visible;
	check for visibility with Window.is_visible().
	*/
	fn is_open(&self) -> bool;

	/**
	If true, the window should be visible
	on screen and have valid dimensions.
	*/
	fn is_visible(&self) -> bool;

	/**
	Opens the window once built.
	*/
	fn open(&self);

	/**
	Hides the window from view.
	If the window hasn't been opened, does nothing.
	*/
	fn hide(&self);

	/**
	Swaps render buffers for this window.
	Returns a Result, so call this with .unwrap().
	*/
	fn swap_buffers(&self) -> Result<(), WindowError>;

	/**
	Sets the callback that handles window events
	sent to this window.
	To actually have this callback be run,
	the window needs to be told to get events
	with poll_events().
	
	The default callback does nothing.
	*/
	fn set_callback(&self, callback: fn(EventType));

	/**
	Retrieves any window events sent to this window.
	*/
	fn poll_events(&self) -> Box<Iterator<Item=&EventType>>;

	/**
	 Returns a list of all of the input devices.
	 This specifes the device's ID, its type,
	 and info on the inputs on the device.
	 */
	fn get_input_devices(&self) -> Vec<Controller>;
}

impl fmt::Debug for Window {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Window {{ title: {} }}", self.title())
	}
}