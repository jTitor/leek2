/*!
 * Types that involve Winit datatypes or structs.
 */
use super::EventType;

use glutin;
use glutin::WindowEvent;

/**
 * Given a Glutin event, returns an EventType enum.
 */
pub fn convert_glutin_event(event: glutin::WindowEvent) -> EventType {
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

