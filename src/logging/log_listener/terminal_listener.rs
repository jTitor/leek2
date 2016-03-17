///Log listener that prints to a terminal's output.
extern crate log;

use logging::log_listener::LogListener;

struct TerminalListener {
	///The terminal we're connected to.
	terminal: u64,
	///The maximum log level to listen to.
	level: LogLevel
}

impl LogListener for TerminalListener {
	fn on_log(&self, record: &LogRecord) {
		//Print this message to standard output.
		//TODO: we should also keep the time this log was entered.
		println!("{} {}: {}",
			record.location().module_path(),
			record.level(),
			record.args());
	}
}