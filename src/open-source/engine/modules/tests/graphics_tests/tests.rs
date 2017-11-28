use leek2::graphics::Device;
use leek2::graphics::GraphicsFactory;
use leek2::platform::current_platform;
use log;

/**
 * Test that the backend returned by Device::default_backend()
 * is supported according to available_backends().
*/
#[test]
fn test_default_backend_is_actually_available() {
	let platform = current_platform();
	let all_backends = Device::available_backends();
	let default_backend = Device::default_backend();
	assert!(all_backends.contains(&default_backend), "Current platform {} has default backend type {}, which doesn't appear to be supported by said platform; supported backends are {}", platform, default_backend, all_backends);
}

/**
 * Test that the default backend can be created
 * without panicking.
 */
#[test]
fn test_default_backend_can_be_created() {
	let backend = GraphicsFactory::new()
		.build()
		.unwrap();

	debug!("Backend type: {}", backend.device.backend_type());
}