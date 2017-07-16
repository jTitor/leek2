/*!
	Some graphics builders return multiple elements,
	such as returning a graphics context
	and window at the same time.
	This abstracts the returned data
	so it can be split into a Device and Window instance.
*/
use super::super::device::Device;
use super::super::window::Window;

#[derive(Debug)]
pub structDeviceWindowBuilderPayload {
	window: &mut Window,
	device: &mut Device
}