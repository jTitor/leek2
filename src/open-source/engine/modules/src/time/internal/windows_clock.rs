use time::{Clock, TimeStamp, DateTime};
#[cfg(windows)]
use winapi::LARGE_INTEGER;
#[cfg(windows)]
use kernel32::{QueryPerformanceCounter, QueryPerformanceFrequency};
#[cfg(windows)]
type WinTimeStamp = LARGE_INTEGER;

//From https://github.com/ajacksified/rusty/blob/master/src/libstd/sys/windows/time.rs
// Computes (value*numer)/denom without overflow, as long as both
// (numer*denom) and the overall result fit into i64 (which is the case
// for our time conversions).
fn mul_div_i64(value: i64, numer: i64, denom: i64) -> i64 {
	let q = value / denom;
	let r = value % denom;
	// Decompose value as (value/denom*denom + value%denom),
	// substitute into (value*numer)/denom and simplify.
	// r < denom, so (denom*numer) is the upper bound of (r*numer)
	q * numer + r * numer / denom
}

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
fn queryPerformanceCounter(outTimestamp: &mut WinTimeStamp) {
	unsafe {
		//Might be better to actually return the result here
		QueryPerformanceCounter(outTimestamp);
	}
}

#[cfg(windows)]
fn query_performance_counter_as_timestamp(inFrequency: LARGE_INTEGER) -> TimeStamp {
	let mut tempWinTimeStamp: WinTimeStamp = 0;
	queryPerformanceCounter(&mut tempWinTimeStamp);
	mul_div_i64(tempWinTimeStamp, 1, inFrequency)
}

#[cfg(windows)]
fn queryPerformanceFrequency(outFrequency: &mut LARGE_INTEGER) {
	unsafe {
		//Might be better to actually return the result here
		QueryPerformanceFrequency(outFrequency);
	}
}

#[cfg(windows)]
impl WindowsClock {
	pub fn new() -> WindowsClock {
		let mut frequency : LARGE_INTEGER = 0;
		queryPerformanceFrequency(&mut frequency);
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
		self.now_timestamp
	}

	fn previous_timestamp(&self) -> TimeStamp {
		self.previous_timestamp
	}

	fn update(&mut self) {
		self.previous_timestamp = self.now_timestamp;
		self.now_timestamp = query_performance_counter_as_timestamp(self.perfcounter_frequency)
	}

	fn clock_start_timestamp(&self) -> TimeStamp {
		self.origin_timestamp
	}
	
	fn clock_start_datetime(&self) -> DateTime {
		self.origin_datetime
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
}