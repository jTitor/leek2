use time::{Clock, TimeStamp, DateTime, TimeElement};
use winapi::{QueryPerformanceCounter, QueryPerformanceFrequency, LARGEINTEGER};

type WinTimeStamp = LARGEINTEGER;

impl TimeElement for WinTimeStamp {
	fn as_seconds(self) -> f64 {
		unimplemented!()
	}

	fn as_milliseconds(self) -> f64 {
		unimplemented!()
	}

	fn as_timestamp(self) -> TimeStamp {
		unimplemented!()
	}
} 

pub struct WindowsClock {
	origin_datetime: DateTime,
	origin_timestamp: WinTimeStamp,
	now_timestamp: WinTimeStamp,
	previous_timestamp: WinTimeStamp,
	//Uses performance counter structs.
	//TODO: add Win32 fields
	perfcounter_frequency: LARGEINTEGER
	
}

fn queryPerformanceCounter(outTimestamp: &mut WinTimeStamp) {
	unsafe {
		//Might be better to actually return the result here
		_ = QueryPerformanceCounter(outTimestamp);
	}
}

fn queryPerformanceFrequency(outFrequency: &mut LARGEINTEGER) {
	unsafe {
		//Might be better to actually return the result here
		_ = QueryPerformanceFrequency(outFrequency);
	}
}

impl WindowsClock {
	pub fn new() -> WindowsClock {
		let frequency : LARGEINTEGER = 0;
		queryPerformanceFrequency(&frequency);
		let mut start_timestamp : WinTimeStamp = 0;
		queryPerformanceCounter(&start_timestamp);
		WindowsClock {
			origin_datetime: DateTime::now(),
			origin_timestamp: start_timestamp,
			now_timestamp: start_timestamp,
			previous_timestamp: start_timestamp,
			perfcounter_frequency: frequency
		}
	}
}

impl Clock for WindowsClock {
	fn now_timestamp(&self) -> TimeStamp {
		self.now_timestamp.as_timestamp()
	}

	fn previous_timestamp(&self) -> TimeStamp {
		self.previous_timestamp.as_timestamp()
	}

	fn update(&mut self) {
		self.previous_timestamp = self.now_timestamp;
		queryPerformanceCounter(&self.now_timestamp)
	}

	fn clock_start_timestamp(&self) -> TimeStamp {
		self.origin_timestamp.as_timestamp()
	}
	
	fn clock_start_datetime(&self) -> DateTime {
		self.origin_datetime
	}
}