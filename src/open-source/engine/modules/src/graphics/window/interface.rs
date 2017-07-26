/*!
	Generic specification for a window.
	Windows are separate from graphics devices,
	but depend on them to perform rendering;
	it's possible to have multiple windows for the
	same application, but the current Game
	implementation doesn't make use of this.
*/

use math::Vec2;
use graphics::Device;

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
	fn swap_buffers(&self) -> Result<>;

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
	If any events were receieved, they will be
	handled by the callback set in set_callback().
	*/
	fn poll_events(&self);

	/**
	Links the given input device to the window.
	The window will receive all input events relating
	to this input device.
	*/
	fn connect_input_device(&self, device_id: u32, type_id: u32);
}