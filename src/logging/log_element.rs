///Represents a single log entry.
use std::fmt;

//TODO: PartialOrd derive implementation is apparently
//very bad, consider alternate route.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum LogSeverity {
	Verbose,
	Debug,
	Info,
	Warning,
	Error
}

impl fmt::Display for LogSeverity {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let disp_text;
		match *self {
			LogSeverity::Verbose => { disp_text = "V"; }
			LogSeverity::Debug => { disp_text = "D"; }
			LogSeverity::Info => { disp_text = "I"; }
			LogSeverity::Warning => { disp_text = "W"; }
			LogSeverity::Error => { disp_text = "E"; }
		}
        write!(f, "{}", disp_text)
    }
}

#[derive(Debug)]
pub struct LogElement {
	pub severity : LogSeverity,
	//Actual log message.
	pub text : String,
	pub tag : String
}

impl fmt::Display for LogElement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.severity, self.tag, self.text)
    }
}