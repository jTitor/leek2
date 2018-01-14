use glutin;
use graphics::WindowError;

impl From<glutin::ContextError> for WindowError {
	fn from(error: glutin::ContextError) -> WindowError {
		//For now, just convert these errors into a
		//"backend failed" error until we can
		//copy the error's reason/cause
		WindowError::BackendOperationFailed
	}
}