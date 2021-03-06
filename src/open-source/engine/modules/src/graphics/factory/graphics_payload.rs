/*!
 Many graphics libraries create the window and
 graphics backends simultaneously, so it isn't
 always practical to return a window or graphics
 device alone. This class contains both objects.
 */
use graphics::{Device, Window};

pub struct GraphicsPayload {
	pub device: Box<Device>,
	pub window: Box<Window>
}