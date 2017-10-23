use platform::{current_platform, PlatformCode};
use time::Clock;
use time::internal::{WindowsClock, PosixClock};
use game::GameError;

pub struct ClockFactory {}

impl ClockFactory {
	pub fn new() -> ClockFactory { ClockFactory{} }
	
	pub fn build(&self) -> Result<&mut Clock, GameError> {
		match current_platform() {
			PlatformCode::Windows => { return Ok(&mut WindowsClock::new()); },
			PlatformCode::MacOS => { return Ok(&mut PosixClock::new()); },
			PlatformCode::Linux => { return Ok(&mut PosixClock::new()); }
			_ => { return Err(GameError::PlatformUnsupported); }
		}
	}
}