use super::internal::{DateTimeInternal, DateTimeInternalFactory};

pub type TimeStamp = i64;
pub type TimeDuration = i64;

pub trait TimeElement {
	fn as_seconds(self) -> f64 {
		self.as_timestamp() as f64 / 1000000000.0
	}

	fn as_milliseconds(self) -> f64 {
		self.as_timestamp() as f64 / 1000000.0
	}

	fn as_timestamp(self) -> TimeStamp;
}

impl TimeElement for i64 {
	fn as_timestamp(self) -> TimeStamp {
		self
	}
}

pub struct TimeRange {
	pub start: TimeStamp,
	pub end: TimeStamp
}

impl TimeRange {
	fn duration(&self) -> TimeDuration {
		self.end - self.start
	}
}

pub struct DateTime {
	pub origin: DateTimeInternal,
	pub offset: TimeStamp	//The time since the given origin.
}

impl DateTime {
	pub fn now() -> DateTime {
		DateTime {
			origin: DateTimeInternalFactory::now(),
			offset: 0
		}
	}

	pub fn offset_by(&self, offset: TimeStamp) -> DateTime {
		DateTime {
			origin: self.origin,
			offset: self.offset + offset
		}
	}
}

//TODO: impl fmt::Display for DateTime