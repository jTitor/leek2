use platform::{current_platform, PlatformCode};
use time::Clock;
use time::internal::{WindowsClock, PosixClock};
use game::GameError;

pub struct ClockFactory {}

impl ClockFactory {
	fn new() -> ClockFactory { ClockFactory{} }
	
	fn build(&self) -> Result<&Clock, GameError> {
		match current_platform() {
			PlatformCode::Windows => { return Ok(&WindowsClock::new()); },
			PlatformCode::MacOS => { return Ok(&PosixClock::new()); },
			PlatformCode::Linux => { return Ok(&PosixClock::new()); }
			_ => { return Err(GameError::PlatformUnsupported); }
		}
	}
}