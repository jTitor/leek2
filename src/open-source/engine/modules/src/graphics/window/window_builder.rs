/*!
Generic specification for window builders.
*/

use math::Vec2;
use graphics::{Device, Window};
use graphics::window::WindowBuilderError;

#[derive(Debug)]
pub struct WindowBuilder {
	title: String,
	dimensions: Vec2,
	position: Vec2,
	vsync: bool
}

impl WindowBuilder {
	pub fn new() -> WindowBuilder {
		WindowBuilder{
			title: "Untitled Window".to_string(),
			dimensions: Vec2::new(1.0,1.0),
			position: Vec2::default(),
			vsync: false
		}
	}

	pub fn build(&self, graphics: &Device) -> Result<Box<Window>, WindowBuilderError> {
		//Dispatch based on the device type.
		unimplemented!();
		Ok(unimplemented!())
	}

	pub fn with_title(&mut self, new_title: &str) -> &mut WindowBuilder {
		self.title = new_title.to_owned();
		self
	}

	pub fn with_dimensions(&mut self, new_dimensions: Vec2) -> &mut WindowBuilder {
		self.dimensions = new_dimensions;
		self
	}

	pub fn with_position(&mut self, new_position: Vec2) -> &mut WindowBuilder {
		self.position = new_position;
		self
	}
}