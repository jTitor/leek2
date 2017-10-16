/*!
 Generic specification for time measurers.
 */
use platform::{current_platform, PlatformCode};
use time::{TimeStamp, TimeDuration};
use super::internal::WindowsClock;
use game::GameError;

pub trait Clock {
	/**
	 Gets the clock's current time.
	 */
	fn now(&self) -> TimeStamp;
	/**
	 Gets the timestamp at which update() was called
	 before now().
	 */
	fn previous_time(&self) -> TimeStamp;
	/**
	 Updates the clock, so that now()
	 returns the current time.
	 */
	fn update(&self);
	/**
	 Returns the time elapsed between previous_time()
	 and now().
	 */
	fn elapsed_time(&self) -> TimeDuration {
		self.now() - self.previous_time()
	}
}

pub trait ClockInternal {
	/**
	 Returns the timestamp when this clock was initialized.
	 This is used to generate DateTimes on this clock
	 by applying the elapsed timestamp as a duration on
	 clock_start_datetime().
	 */
	fn clock_start_timestamp(&self) -> TimeStamp {
		unimplemented!()
	}
	
	/**
	 Returns the timezone-dependent date and time
	 this clock was initialized.
	 This is used to generate DateTimes on this clock
	 by applying the elapsed timestamp as a duration.
	 */
	fn clock_start_datetime(&self) {

	}
}

pub struct ClockFactory {}

impl ClockFactory {
	fn new() -> ClockFactory { ClockFactory{} }
	
	fn build(&self) -> Result<&Clock, GameError> {
		match current_platform() {
			PlatformCode::Windows => { return Ok(&WindowsClock::new()); },
			PlatformCode::MacOS => { return Ok(unimplemented!()); },
			PlatformCode::Linux => { return Ok(unimplemented!()); }
			_ => { return Err(GameError::PlatformUnsupported); }
		}
	}
}