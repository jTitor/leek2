use chrono;
use time::TimeStamp;

pub type DateTimeInternal = chrono::DateTime<chrono::Local>;

pub struct DateTimeInternalFactory {}

impl DateTimeInternalFactory {
	pub fn now() -> DateTimeInternal {
		chrono::Local::now()
	}

	/**
	 Returns a DateTimeInternal representing the given DateTime's date and offset. 
	*/
	pub fn normalize(dt: DateTimeInternal, offset: TimeStamp) -> DateTimeInternal {
		let duration = chrono::Duration::nanoseconds(offset);
		let result = dt.checked_add_signed(duration);
		match result {
			Some(t) => { return t; },
			_ => {
				assert!(false, "Failed to unwrap time `{}`! Dropping timestamp offset `{}`", dt, offset);
				return dt;
			}
		}
	}
}