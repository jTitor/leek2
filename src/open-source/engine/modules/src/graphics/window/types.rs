/*!
Specifies enums and types used by the other
modules in this module.
*/
use glutin;
use glutin::WindowEvent;

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
	ReceivedCharacter(char),
	MouseMoved((f64, f64)),
	//Touch(Touch)
	Unknown
}

pub fn convert_winit_event(event: glutin::WindowEvent) -> EventType {
	match event {
		glutin::WindowEvent::Resized(new_width, new_height) => { return EventType::Resized(new_width, new_height); },
		glutin::WindowEvent::Moved(x, y) => { return EventType::Moved(x, y); },
		glutin::WindowEvent::Closed => { return EventType::Closed; },
		glutin::WindowEvent::ReceivedCharacter(char_received) => { return EventType::ReceivedCharacter(char_received); },
		glutin::WindowEvent::Focused(is_focused) => { return EventType::Focused(is_focused); },
		//glutin::WindowEvent::KeyboardInput(key_state, scan_code, virtual_key) => { return EventType::KeyboardInput(key_state, scan_code, virtual_key); },
		glutin::WindowEvent::MouseMoved{ device_id, position } => { return EventType::MouseMoved(position); },
		//glutin::WindowEvent::Awakened => { return EventType::Awakened; },
		glutin::WindowEvent::Refresh => { return EventType::Refresh; },
		glutin::WindowEvent::Suspended(is_suspended) => { return EventType::Suspended(is_suspended); },
		_ => { return EventType::Unknown; }
	}
}