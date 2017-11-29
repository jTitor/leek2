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
			LogSeverity::Verbose => { disp_text = "V"; },
			LogSeverity::Debug => { disp_text = "D"; },
			LogSeverity::Info => { disp_text = "I"; },
			LogSeverity::Warning => { disp_text = "W"; },
			LogSeverity::Error => { disp_text = "E"; }
		}
		write!(f, "{}", disp_text)
	}
}

impl LogSeverity {
	pub fn long_name(self) -> &'static str {
		let disp_text;
		match self {
			LogSeverity::Verbose => { disp_text = "verbose"; },
			LogSeverity::Debug => { disp_text = "debug"; },
			LogSeverity::Info => { disp_text = "info"; },
			LogSeverity::Warning => { disp_text = "warning"; },
			LogSeverity::Error => { disp_text = "error"; }
		}

		disp_text
	}

	pub fn as_prefix(self) -> &'static str {
		let disp_text;
		match self {
			LogSeverity::Verbose => { disp_text = "(verbose) "; },
			LogSeverity::Debug => { disp_text = "(debug) "; },
			LogSeverity::Info => { disp_text = ""; },
			LogSeverity::Warning => { disp_text = "(warning) "; },
			LogSeverity::Error => { disp_text = "(error) "; }
		}

		disp_text
	}
}