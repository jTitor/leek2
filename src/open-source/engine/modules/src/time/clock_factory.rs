use platform::{current_platform, PlatformCode};
use time::Clock;
use time::internal::{WindowsClock, PosixClock};
use game::GameError;

pub struct ClockFactory {}

impl ClockFactory {
	pub fn new() -> ClockFactory { ClockFactory{} }
	
	pub fn build(&self) -> Result<Box<Clock>, GameError> {
		match current_platform() {
			PlatformCode::Windows => { return Ok(Box::new(WindowsClock::new())); },
			PlatformCode::MacOS => { return Ok(Box::new(PosixClock::new())); },
			PlatformCode::Linux => { return Ok(Box::new(PosixClock::new())); }
			_ => { return Err(GameError::PlatformUnsupported); }
		}
	}
}