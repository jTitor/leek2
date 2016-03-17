///Log listener that prints to a file.
extern crate log;

use logging::log_listener::LogListener;

struct FileListener {
	///The file we're connected to.
	file: u64,
	///The maximum log level to listen to.
	level: LogLevel
}

impl LogListener for FileListener {
	fn on_log(&self, record: &LogRecord) {
		//Print this message to the file.
		//TODO: we should also keep the time this log was entered.
		unimplemented!();
		println!("{} {}: {}",
			record.location().module_path(),
			record.level(),
			record.args());
	}
}