use leek2::graphics::available_backends;
use leek2::graphics::Device;

/*
#[test]
fn test_graphics() {
	unimplemented!()
}
*/

/*!
Test that the backend returned by Device::default_backend()
is supported according to available_backends().
*/
#[test]
fn test_default_backend_is_actually_available() {
	let all_backends = available_backends();
	let default_backend = Device::default_backend();
	assert!(all_backends.contains(&default_backend));
}