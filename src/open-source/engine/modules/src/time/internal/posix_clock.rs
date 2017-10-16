use time::{Clock, TimeStamp, DateTime};

pub struct PosixClock {
	origin_datetime: DateTime,
	origin_timestamp: TimeStamp,
	//Uses performance counter structs.
	now_timestamp: TimeStamp,
	previous_timestamp: TimeStamp
}

impl PosixClock {
	pub fn new() -> PosixClock {
		let start_timestamp = unimplemented!();
		PosixClock {
			origin: DateTime::now(),
			origin_timestamp: start_timestamp,
			now_timestamp: start_timestamp,
			previous_timestamp: start_timestamp
		}
	}
}

impl Clock for PosixClock {
	fn now_timestamp(&self) -> TimeStamp {
		//Query the performance counter.
		self.now_timestamp
	}

	fn previous_timestamp(&self) -> TimeStamp {
		//Query the performance counter.
		self.previous_timestamp
	}

	fn update(&self) {
		//Query the performance counter.
		unimplemented!()
	}

	fn clock_start_timestamp(&self) -> TimeStamp {
		self.origin_timestamp
	}
	
	fn clock_start_datetime(&self) -> DateTime {
		self.origin_datetime
	}
}