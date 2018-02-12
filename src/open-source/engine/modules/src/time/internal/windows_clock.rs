use time::{Clock, TimeStamp, DateTime, ClockType};
#[cfg(windows)]
use std::mem;
#[cfg(windows)]
use math::scalar::mul_div_i64;
#[cfg(windows)]
use winapi::um::winnt::LARGE_INTEGER;
#[cfg(windows)]
use kernel32::{QueryPerformanceCounter, QueryPerformanceFrequency};

#[cfg(windows)]
pub struct WindowsClock {
	origin_datetime: DateTime,
	origin_timestamp: TimeStamp,
	now_timestamp: TimeStamp,
	previous_timestamp: TimeStamp,
	//Uses performance counter structs.
	//TODO: add Win32 fields
	perfcounter_frequency: LARGE_INTEGER
}

#[cfg(windows)]
/**
 * Stores the performance counter's current value
 * in the given LARGE_INTEGER.
 */
fn query_performance_counter(out_timestamp: &mut LARGE_INTEGER) {
	unsafe {
		//Might be better to actually return the result here
		QueryPerformanceCounter(out_timestamp.QuadPart_mut());
	}
}

#[cfg(windows)]
fn query_performance_counter_as_timestamp(in_frequency: LARGE_INTEGER) -> TimeStamp {
	let mut temp_timestamp: LARGE_INTEGER = unsafe { mem::zeroed() };
	query_performance_counter(&mut temp_timestamp);
	
	//A little gross since the UNION! macro
	//makes getters/setters methods, but
	//this matches the actual Win32 fields
	//on a LARGE_INTEGER
	let perf_counter = unsafe { *temp_timestamp.QuadPart() };
	let perf_frequency = unsafe { *in_frequency.QuadPart() };

	mul_div_i64(perf_counter, 1, perf_frequency)
}

#[cfg(windows)]
fn query_performance_frequency(out_frequency: &mut LARGE_INTEGER) {
	unsafe {
		//Might be better to actually return the result here
		QueryPerformanceFrequency(out_frequency.QuadPart_mut());
	}
}

#[cfg(windows)]
impl WindowsClock {
	pub fn new() -> WindowsClock {
		let mut frequency : LARGE_INTEGER = unsafe { mem::zeroed() };
		query_performance_frequency(&mut frequency);
		assert!(unsafe { *frequency.QuadPart() } > 0, "WindowsClock: Clock frequency isn't nonzero, can't get actual performance counter values");
		let start_timestamp = query_performance_counter_as_timestamp(frequency);
		WindowsClock {
			origin_datetime: DateTime::now(),
			origin_timestamp: start_timestamp,
			now_timestamp: start_timestamp,
			previous_timestamp: start_timestamp,
			perfcounter_frequency: frequency
		}
	}
}

#[cfg(windows)]
impl Clock for WindowsClock {
	fn now_timestamp(&self) -> TimeStamp {
		self.now_timestamp - self.origin_timestamp
	}

	fn previous_timestamp(&self) -> TimeStamp {
		self.previous_timestamp - self.origin_timestamp
	}

	fn update(&mut self) {
		self.previous_timestamp = self.now_timestamp;
		self.now_timestamp = query_performance_counter_as_timestamp(self.perfcounter_frequency);
	}

	fn clock_start_timestamp(&self) -> TimeStamp {
		self.origin_timestamp
	}
	
	fn clock_start_datetime(&self) -> DateTime {
		self.origin_datetime
	}

	fn clock_type(&self) -> ClockType {
		ClockType::WindowsClock
	}
}

//Begin dummy section.
//If we're not on Win32, WindowsClock should build as a no-op.
#[cfg(not(windows))]
pub struct WindowsClock {}

#[cfg(not(windows))]
impl WindowsClock {
	pub fn new() -> WindowsClock {
		WindowsClock {}
	}
}

#[cfg(not(windows))]
impl Clock for WindowsClock {
	fn now_timestamp(&self) -> TimeStamp { 0 }

	fn previous_timestamp(&self) -> TimeStamp { 0 }

	fn update(&mut self) {}

	fn clock_start_timestamp(&self) -> TimeStamp { 0 }
	
	fn clock_start_datetime(&self) -> DateTime { DateTime::now() }

	fn clock_type(&self) -> ClockType {
		ClockType::WindowsClock
	}
}