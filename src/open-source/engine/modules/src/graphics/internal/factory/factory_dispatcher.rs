/*!
 Given a generic Factory, determines
 which Device and Window builder needs to be called
 to generate the desired payload.
 */
use graphics::{GraphicsFactory, GraphicsPayload, BackendError, BackendType};
use super::glutin::GlutinDeviceWindowBuilder;
use std::rc::Rc;

pub struct FactoryDispatcher {
	pub factory: Rc<GraphicsFactory>
}

impl FactoryDispatcher {
	pub fn new(factory: &GraphicsFactory) -> FactoryDispatcher {
		FactoryDispatcher { factory: Rc::new(*factory) }
	}

	pub fn dispatch(&self) -> Result<GraphicsPayload, BackendError> {
		match self.factory.device_request.device_type {
			BackendType::OpenGL => {
				//Try to get the Glutin factories.
				let dispatcher = GlutinDeviceWindowBuilder::new();
				return Ok(dispatcher.build(self));
			}
			_ => { return Err(BackendError::NoneAvailable); }
		}
	}
}