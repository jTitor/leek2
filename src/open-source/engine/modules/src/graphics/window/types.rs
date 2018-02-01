/*!
Specifies enums and types used by the other
modules in this module.
*/
use winit;

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
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Visibility {
	Closed,
	Normal,
	Minimized,
	Maximized,
	Fullscreen
}

/**
General specification for window events.
This is as close to the Glium event types
as possible, except input is handled separately.
*/
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EventType {
	Awakened,
	Resized(u32, u32),
	Moved(i32, i32),
	Closed,
	//TODO: DroppedFile and Touch
	//need parameters implemented
	//DroppedFile(PathBuf),
	Focused(bool),
	Refresh,
	Suspended(bool),
	//TODO: replace these with input-generic
	//events
	Keyboard(),
	MouseMoved((u32, u32)),
	//Touch(Touch)
	Unknown
}

fn convert_winit_event(event: winit::Event) -> EventType {
	winit::Event::Resized(new_width, new_height) => { return EventType::Resized(new_width, new_height); },
	winit::Event::Moved(x, y) => { return EventType::Moved(x, y); },
	winit::Event::Closed => { return EventType::Closed; },
	winit::Event::ReceivedCharacter(char_received) => { return EventType::ReceivedCharacter(char_received); },
	winit::Event::Focused(is_focused) => { return EventType::Focused(is_focused); },
	winit::Event::KeyboardInput(key_state, scan_code, virtual_key) => { return EventType::KeyboardInput(key_state, scan_code, virtual_key); },
	winit::Event::MouseMoved((x, y)) => { return EventType::MouseMoved((x, y)); },
	winit::Event::Awakened => { return EventType::Awakened; },
	winit::Event::Refresh => { return EventType::Refresh; },
	winit::Event::Suspended(is_suspended) => { return EventType::Suspended(is_suspended); },
	_ = { return EventType::Unknown; }
}

/**
Windows call their callback whenever they get a window event.
*/
type WindowEventCallback = fn(EventType);