use time::{Clock, TimeStamp};

pub struct PosixClock {
	//Uses performance counter structs.
}

impl PosixClock {
	pub fn new() -> PosixClock {
		PosixClock {}
	}
}

impl Clock for PosixClock {
	fn now_timestamp(&self) -> TimeStamp {
		//Query the performance counter.
		unimplemented!()
	}

	fn previous_timestamp(&self) -> TimeStamp {
		//Query the performance counter.
		unimplemented!()
	}

	fn update(&self) {
		//Query the performance counter.
		unimplemented!()
	}

	fn clock_start_timestamp(&self) -> TimeStamp {
		unimplemented!()
	}
	
	fn clock_start_datetime(&self) -> DateTime {
		unimplemented!()
	}
}