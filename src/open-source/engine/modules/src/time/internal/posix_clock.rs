use std::time::{Instant, Duration};
use time::{Clock, TimeStamp, DateTime, ClockType};

pub struct PosixClock {
	origin_datetime: DateTime,
	origin_timestamp: Instant,
	now_timestamp: Instant,
	previous_timestamp: Instant
}

impl PosixClock {
	pub fn new() -> PosixClock {
		let start_timestamp = Instant::now();
		PosixClock {
			origin_datetime: DateTime::now(),
			origin_timestamp: start_timestamp,
			now_timestamp: start_timestamp,
			previous_timestamp: start_timestamp
		}
	}
}

fn duration_to_timestamp(duration: Duration) -> TimeStamp {
	let seconds_as_nanos = duration.as_secs() as i64 * 1000000000;
	let nanos_remaining = duration.subsec_nanos() as i64;
	
	seconds_as_nanos + nanos_remaining
}

impl Clock for PosixClock {
	fn now_timestamp(&self) -> TimeStamp {
		//Query the performance counter.
		duration_to_timestamp(self.now_timestamp.duration_since(self.origin_timestamp))
	}

	fn previous_timestamp(&self) -> TimeStamp {
		//Query the performance counter.
		duration_to_timestamp(self.previous_timestamp.duration_since(self.origin_timestamp))
	}

	fn update(&mut self) {
		//Query the performance counter.
		self.previous_timestamp = self.now_timestamp;
		self.now_timestamp = Instant::now();
	}

	fn clock_start_timestamp(&self) -> TimeStamp {
		duration_to_timestamp(self.origin_timestamp.duration_since(self.origin_timestamp))
	}
	
	fn clock_start_datetime(&self) -> DateTime {
		self.origin_datetime
	}
	
	fn clock_type(&self) -> ClockType {
		ClockType::PosixClock
	}
}