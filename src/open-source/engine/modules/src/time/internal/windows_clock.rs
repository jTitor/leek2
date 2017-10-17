use time::{Clock, TimeStamp, DateTime};

pub struct WindowsClock {
	origin_datetime: DateTime,
	origin_timestamp: TimeStamp,
	//Uses performance counter structs.
	//TODO: add Win32 fields
	now_timestamp: TimeStamp,
	previous_timestamp: TimeStamp
}

impl WindowsClock {
	pub fn new() -> WindowsClock {
		let start_timestamp = unimplemented!();
		WindowsClock {
			origin_datetime: DateTime::now(),
			origin_timestamp: start_timestamp,
			now_timestamp: start_timestamp,
			previous_timestamp: start_timestamp
		}
	}
}

impl Clock for WindowsClock {
	fn now_timestamp(&self) -> TimeStamp {
		//Query the performance counter.
		self.now_timestamp
	}

	fn previous_timestamp(&self) -> TimeStamp {
		//Query the performance counter.
		self.previous_timestamp
	}

	fn update(&mut self) {
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