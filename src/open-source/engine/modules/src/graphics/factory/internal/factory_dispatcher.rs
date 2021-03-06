/*!
 Given a generic Factory, determines
 which Device and Window builder needs to be called
 to generate the desired payload.
 */
use graphics::{GraphicsFactory, GraphicsPayload, BackendError, BackendType};
use graphics::available_backends;
use platform::current_platform;
use super::glutin::GlutinDeviceWindowBuilder;

pub struct FactoryDispatcher {
	pub factory: GraphicsFactory
}

impl FactoryDispatcher {
	pub fn new(factory: &GraphicsFactory) -> FactoryDispatcher {
		FactoryDispatcher { factory: factory.clone() }
	}

	pub fn dispatch(&self) -> Result<GraphicsPayload, BackendError> {
		//Check that the backend is available on this platform.
		let backend_request_type = self.factory.device_request.device_type;
		let backends = available_backends();
		let platform = current_platform();
		if !backends.contains(&backend_request_type) {
			return Err(BackendError::BackendUnavailable(backend_request_type, platform));
		}
		
		match backend_request_type {
			BackendType::OpenGL => {
				//Try to get the Glutin factories.
				let dispatcher = GlutinDeviceWindowBuilder::new();
				return Ok(dispatcher.build(self));
			}
			_ => { return Err(BackendError::NoneAvailable); }
		}
	}
}