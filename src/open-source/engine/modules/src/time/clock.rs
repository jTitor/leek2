/*!
 Generic specification for time measurers.
 */
use platform::{current_platform, PlatformCode};
use time::{TimeStamp, TimeDuration, DateTime};
use super::internal::{WindowsClock, PosixClock};
use game::GameError;

pub trait Clock {
	/**
	 Gets the clock's current time.
	 */
	fn now_timestamp(&self) -> TimeStamp;
	/**
	 Gets the timestamp at which update() was called
	 before now().
	 */
	fn previous_timestamp(&self) -> TimeStamp;
	/**
	 Updates the clock, so that now_timestamp()
	 returns the current time.
	 */
	fn update(&mut self);
	/**
	 Returns the timestamp when this clock was initialized.
	 This is used to generate DateTimes on this clock
	 by applying the elapsed timestamp as a duration on
	 clock_start_datetime().
	 */
	fn clock_start_timestamp(&self) -> TimeStamp;
	
	/**
	 Returns the timezone-dependent date and time
	 this clock was initialized.
	 This is used to generate DateTimes on this clock
	 by applying the elapsed timestamp as a duration.
	 */
	fn clock_start_datetime(&self) -> DateTime;

	/**
	 Returns the time elapsed between previous_time()
	 and now().
	 */
	fn elapsed_time(&self) -> TimeDuration {
		self.now_timestamp() - self.previous_timestamp()
	}

	fn timestamp_as_datetime(&self, timestamp: TimeStamp) -> DateTime {
		self.clock_start_datetime().offset_by(timestamp - self.clock_start_timestamp())
	}

	fn now_datetime(&self) -> DateTime {
		self.timestamp_as_datetime(self.now_timestamp())
	}

	fn previous_datetime(&self) -> DateTime {
		self.timestamp_as_datetime(self.previous_timestamp())
	}
}