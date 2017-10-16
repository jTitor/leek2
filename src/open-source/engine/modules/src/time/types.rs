pub type TimeStamp = i64;

impl TimeStamp {
	fn as_seconds(self) -> f64 {
		unimplemented!()
	}

	fn as_milliseconds(self) -> f64 {
		unimplemented!()
	}
}

pub type TimeDuration = i64;

impl TimeDuration {
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