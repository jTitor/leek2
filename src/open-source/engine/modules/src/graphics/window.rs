/*!
	Generic specification for a window.
	Windows are separate from graphics devices,
	but depend on them to perform rendering;
	it's possible to have multiple windows for the
	same application, but the current Game
	implementation doesn't make use of this.
*/

use super::super::math::linear_algebra::vec2::Vec2;
use super::device::Device;

/**
Specifies the window's state on screen.

Values:
  * Closed: The window has been closed. Specifically,
  the window is not in the windowing manager's list of
  accessible windows, and it is a runtime error
  for the window to be visible on the screen in this state.
  * Normal: The window is in the windowing manager's
  list of accessible windows and is visible on the screen.
  * Minimized: The window is in the windowing manager's
  list of accessible windows, but is not visible on
  the screen. Any fields associated with window position
  or size are invalid when the window is in this state.
  * Maximized: The window is in the windowing manager's
  list of accessible windows and covers as much of the screen as the manager will allow.
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
Windows can't be 
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
	fn open(&self) -> Result<>;

	/**
	Hides the window from view.
	If the window hasn't been opened, does nothing.
	*/
	fn hide(&self) -> Result<>;

	/**
	Swaps render buffers for this window.
	Returns a Result, so call this with .unwrap().
	*/
	fn swap_buffers(&self) -> Result<>;
}

/**
Generic specification for window builders.
*/
pub struct WindowBuilderInfo {
	title: String
	dimensions: Vec2,
	position: Vec2,
	vsync: bool
}

impl WindowBuilder {
	pub fn new() -> WindowBuilder {
		WindowBuilder{
			title: "Untitled Window".to_string(),
			dimensions: Vec2::new(1,1),
			position: Vec2{},
			vsync: false
		}
	}

	pub fn build(&self, graphics: &Device) -> Result<&Window, Error> {
		//Dispatch based on the device type.
		unimplemented!();
		Ok(unimplemented!())
	}

	pub fn with_title(&mut self, newTitle: &str) -> &mut WindowBuilder {
		self.title = newTitle;
		self
	}

	pub fn with_dimensions(&mut self, newDimensions: Vec2) -> &mut WindowBuilder {
		self.dimensions = newDimensions;
		self
	}

	pub fn with_position(&mut self, newPosition: Vec2) -> &mut WindowBuilder {
		self.position = newPosition;
		self
	}
}