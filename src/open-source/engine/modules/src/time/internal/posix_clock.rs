use std::time::{Instant, Duration};
use time::{Clock, TimeStamp, DateTime};

pub struct PosixClock {
	origin_datetime: DateTime,
	origin_timestamp: Instant,
	//Uses performance counter structs.
	now_timestamp: Instant,
	previous_timestamp: Instant
}

impl PosixClock {
	pub fn new() -> PosixClock {
		let start_timestamp = Instant::now();
		PosixClock {
			origin: DateTime::now(),
			origin_timestamp: start_timestamp,
			now_timestamp: start_timestamp,
			previous_timestamp: start_timestamp
		}
	}
}

//TODO: These are second + nanosecond structs,
//convert them to be entirely nanosecond
impl Clock for PosixClock {
	fn now_timestamp(&self) -> TimeStamp {
		//Query the performance counter.
		self.now_timestamp.duration_since(self.origin_timestamp)
	}

	fn previous_timestamp(&self) -> TimeStamp {
		//Query the performance counter.
		self.previous_timestamp.duration_since(self.origin_timestamp)
	}

	fn update(&mut self) {
		//Query the performance counter.
		self.previous_timestamp = self.now_timestamp;
		self.now_timestamp = Instant::now();
	}

	fn clock_start_timestamp(&self) -> TimeStamp {
		self.origin_timestamp.duration_since(self.origin_timestamp)
	}
	
	fn clock_start_datetime(&self) -> DateTime {
		self.origin_datetime
	}
}