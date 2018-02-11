/*!
 * Types that involve Winit datatypes or structs.
 */
use super::EventType;

use winit;
use winit::Event;

pub fn convert_winit_event(winit_event: winit::Event) -> EventType {
	match winit_event {
		//Events involving a specific window.
		winit::Event::WindowEvent { window_id, event } => { return convert_window_event(window_id, event); },
		//Events associated with an input device,
		//but not with a specific window.
		winit::Event::DeviceEvent { device_id, event } => { return convert_device_event(device_id, event); },
		_ => { return EventType::Unknown; }
		//These aren't implemented, so just return unknown
		//event
		//App started/gained focus
		// winit::Event::Awakened => { return EventType::Unknown; },
		//App suspended/lost focus
		//Parameter is true if the app became suspended,
		//and false if it stopped being suspended
		// winit::Event::Suspended(bool) => { return EventType::Unknown; }
	}
}

/**
 * Given a Glutin event, returns an EventType enum.
 */
pub fn convert_window_event(_id: winit::WindowId, event: winit::WindowEvent) -> EventType {
	match event {
		winit::WindowEvent::Resized(new_width, new_height) => { return EventType::Resized(new_width, new_height); },
		winit::WindowEvent::Moved(x, y) => { return EventType::Moved(x, y); },
		winit::WindowEvent::Closed => { return EventType::Closed; },
		winit::WindowEvent::ReceivedCharacter(char_received) => { return EventType::ReceivedCharacter(char_received); },
		winit::WindowEvent::Focused(is_focused) => { return EventType::Focused(is_focused); },
		//winit::WindowEvent::KeyboardInput(key_state, scan_code, virtual_key) => { return EventType::KeyboardInput(key_state, scan_code, virtual_key); },
		//winit::WindowEvent::MouseMoved{ device_id, position } => { return EventType::MouseMoved(position); },
		//winit::WindowEvent::Awakened => { return EventType::Awakened; },
		winit::WindowEvent::Refresh => { return EventType::Refresh; },
		_ => { return EventType::Unknown; }
	}
}

pub fn convert_device_event(_id: winit::DeviceId, _event: winit::DeviceEvent) -> EventType {
	//TODO: winit device events need implementing
	EventType::Unknown
}
