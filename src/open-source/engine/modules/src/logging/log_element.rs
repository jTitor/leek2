/*!
	Represents a single log entry.
*/
use std::fmt;
use super::log_severity::LogSeverity;
use time::DateTime;

#[derive(Debug)]
pub struct LogElement<'a> {
	pub severity : LogSeverity,
	//Actual log message.
	pub text : &'a str,
	pub tag : &'a str,
	pub time_stamp : DateTime
}

impl<'a> fmt::Display for LogElement<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {}, {}, {})", self.time_stamp, self.severity, self.tag, self.text)
	}
}