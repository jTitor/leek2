use super::internal::DateTimeInternalFactory;

pub type TimeStamp = i64;
pub type TimeDuration = i64;

trait TimeElement {
	fn as_seconds(self) -> f64;/* {
		unimplemented!()
	}*/

	fn as_milliseconds(self) -> f64;/* {
		unimplemented!()
	}*/
}

impl TimeElement for i64 {
	fn as_seconds(self) -> f64 {
		unimplemented!()
	}

	fn as_milliseconds(self) -> f64 {
		unimplemented!()
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
	fn now() -> DateTime {
		DateTime {
			origin: DateTimeInternalFactory::now(),
			offset: 0
		}
	}

	fn offset_by(&self, offset: TimeStamp) -> DateTime {
		DateTime {
			origin: self.origin,
			offset: self.offset + offset
		}
	}
}

//TODO: impl fmt::Display for DateTime