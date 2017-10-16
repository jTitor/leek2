use super::GraphicsPayload;
use super::{BackendType, BackendError};
use math::Vec2;
use super::internal::FactoryDispatcher;

#[derive(Debug)]
pub struct DeviceRequest {
	pub device_type: BackendType
}

#[derive(Debug)]
pub struct WindowRequest {
	pub title: String,
	pub dimensions: Vec2,
	pub position: Vec2,
	pub vsync: bool
}

#[derive(Debug)]
pub struct GraphicsFactory {
	pub device_request: DeviceRequest,
	pub window_request: WindowRequest
}

impl GraphicsFactory {
	fn new() -> GraphicsFactory {
		let device = DeviceRequest {
			device_type: BackendType::Other
		};
		let window = WindowRequest {
			title: String::from("Default Title"),
			dimensions: Vec2::new(1.0, 1.0),
			position: Vec2::new(0.0, 0.0),
			vsync: true
		};
		
		GraphicsFactory {
			device_request: device,
			window_request: window
		}
	}

	fn with_device_type(&mut self, device_type: BackendType) -> &mut GraphicsFactory {
		self.device_request.device_type = device_type;
		self
	}

	fn with_title(&mut self, title: &str) -> &mut GraphicsFactory {
		self.window_request.title = String::from(title);
		self
	}

	fn with_dimensions(&mut self, dimensions: Vec2) -> &mut GraphicsFactory {
		self.window_request.dimensions = dimensions;
		self
	}

	fn with_position(&mut self, position: Vec2) -> &mut GraphicsFactory {
		self.window_request.position = position;
		self
	}

	fn with_vsync(&mut self, vsync: bool) -> &mut GraphicsFactory {
		self.window_request.vsync = vsync;
		self
	}

	fn build(&self) -> Result<GraphicsPayload, BackendError> {
		FactoryDispatcher::new(self)
			.dispatch()
	}
}