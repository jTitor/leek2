/*!
Generic specification for window builders.
*/

use math::Vec2;
use graphics::Device;

#[derive(Debug)]
pub structWindowBuilder {
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
		self.title = newTitle.to_owned();
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