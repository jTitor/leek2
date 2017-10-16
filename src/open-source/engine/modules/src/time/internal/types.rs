use chrono;

pub type DateTimeInternal = chrono::DateTime<chrono::Local>;

pub struct DateTimeInternalFactory {}

impl DateTimeInternalFactory {
	fn now() -> DateTimeInternal {
		chrono::Local::now()
	}
}