///Represents a single log entry.

pub enum LogSeverity {
	Verbose,
	Debug,
	Info,
	Warning,
	Error
}

pub struct LogElement {
	pub severity : LogSeverity,
	//Actual log message.
	pub text : String,
	pub tag : String
}