use std::error::Error;
use std::panic;
use leek2::graphics;
use leek2::graphics::GraphicsFactory;
use leek2::platform::{current_platform, PlatformCode};
use log;

/**
 * Test that the backend returned by Device::default_backend()
 * is supported according to available_backends().
*/
#[test]
fn test_default_backend_is_actually_available() {
	let platform = current_platform();
	let all_backends = graphics::available_backends();
	let default_backend = graphics::default_backend();
	assert!(all_backends.contains(&default_backend), "Current platform {} has default backend type {}, which doesn't appear to be supported by said platform; supported backends are {:?}", platform, default_backend, all_backends);
}

/**
 * Test that the default backend can be created
 * without panicking.
 */
#[test]
fn test_default_backend_can_be_created() {
	let mut error_msg: &str = "";
	let backend_result = panic::catch_unwind(|| {
		GraphicsFactory::new().build()
	});
	let mut did_pass = error_msg != "";

	if let Err(error) = backend_result {
		did_pass = false;
		if let Some(error_msg_raw) = error.downcast_ref::<&'static str>() {
			error_msg = *error_msg_raw;
		}
		//Match the error type.
		match current_platform() {
			PlatformCode::MacOS => {
				//If it complained about not being able to open a window,
				//that's ok for this test; that
				//means the graphics backend
				//did build properly at least
				did_pass = error_msg == "Windows can only be created on the main thread on macOS";
				},
			_ => {}
		}
	}

	//Report any failure.
	assert!(did_pass, "{}", error_msg);
}