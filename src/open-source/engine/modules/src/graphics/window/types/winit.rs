/*!
 * Types that involve Winit datatypes or structs.
 */
use super::EventType;

//Alias gfx's glutin alias to the necessary structs.
use gfx_backend_gl::glutin::{Event, WindowId, WindowEvent, DeviceId, DeviceEvent};

pub fn convert_winit_event(winit_event: Event) -> EventType {
	match winit_event {
		//Events involving a specific window.
		Event::WindowEvent { window_id, event } => { return convert_window_event(window_id, event); },
		//Events associated with an input device,
		//but not with a specific window.
		Event::DeviceEvent { device_id, event } => { return convert_device_event(device_id, event); },
		_ => { return EventType::Unknown; }
		//These aren't implemented, so just return unknown
		//event
		//App started/gained focus
		// Event::Awakened => { return EventType::Unknown; },
		//App suspended/lost focus
		//Parameter is true if the app became suspended,
		//and false if it stopped being suspended
		// Event::Suspended(bool) => { return EventType::Unknown; }
	}
}

/**
 * Given a Glutin event, returns an EventType enum.
 */
pub fn convert_window_event(_id: WindowId, event: WindowEvent) -> EventType {
	match event {
		WindowEvent::Resized(new_width, new_height) => { return EventType::Resized(new_width, new_height); },
		WindowEvent::Moved(x, y) => { return EventType::Moved(x, y); },
		WindowEvent::Closed => { return EventType::Closed; },
		WindowEvent::ReceivedCharacter(char_received) => { return EventType::ReceivedCharacter(char_received); },
		WindowEvent::Focused(is_focused) => { return EventType::Focused(is_focused); },
		//WindowEvent::KeyboardInput(key_state, scan_code, virtual_key) => { return EventType::KeyboardInput(key_state, scan_code, virtual_key); },
		//WindowEvent::MouseMoved{ device_id, position } => { return EventType::MouseMoved(position); },
		//WindowEvent::Awakened => { return EventType::Awakened; },
		WindowEvent::Refresh => { return EventType::Refresh; },
		_ => { return EventType::Unknown; }
	}
}

pub fn convert_device_event(_id: DeviceId, _event: DeviceEvent) -> EventType {
	//TODO: winit device events need implementing
	EventType::Unknown
}
