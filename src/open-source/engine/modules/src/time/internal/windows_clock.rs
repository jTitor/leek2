use time::{Clock, TimeStamp};

pub struct WindowsClock {
	//Uses performance counter structs.
	//TODO: add Win32 fields
}

impl WindowsClock {
	pub fn new() -> WindowsClock {
		WindowsClock {}
	}
}

impl Clock for WindowsClock {
	fn now(&self) -> TimeStamp {
		//Query the performance counter.
		unimplemented!()
	}

	fn previous_time(&self) -> TimeStamp {
		//Query the performance counter.
		unimplemented!()
	}

	fn update(&self) {
		//Query the performance counter.
		unimplemented!()
	}
}