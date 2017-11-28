use leek2::time::{ClockFactory, Clock, ClockType};

#[test]
fn test_clock_factory() {
	//Test:
	//ClockFactory makes the right clock for the platform.
	let clock = ClockFactory::new()
		.build().unwrap();

	if cfg!(windows) {
		assert!(clock.clock_type() == ClockType::WindowsClock, "Clock should be a WindowsClock but isn't")
	}
	else {
		assert!(clock.clock_type() == ClockType::PosixClock, "Clock should be a PosixClock but isn't")
	}
}