///Interface for things that display/respond to log entries.
extern crate log;

use log::{LogRecord, LogLevel, LogMetadata};
use std::io::Write;

///Base class for log listeners.
///Has a connection to some output stream
///and a maximum acceptable log level for filtering.
struct ListenerBase {
	///The output file we're connected to.
	output: &Write,
	///The maximum log level to listen to.
	level: LogLevel,
	///If true, output is connected and we can write
	///to it; otherwise the output is not connected
	///and attempts to write may cause a panic.
	output_ready: bool
}

trait LogListener {
	///Called to initialize the listener.
	fn startup(&self);
	///Called when a Logger has an entry for this listener to display.
	fn on_log(&self, record: &LogRecord);
	///Called to shut down the listener.
	fn shutdown(&self);
}

impl LogListener for ListenerBase {
	///Called when a Logger has an entry for this listener to display.
	fn on_log(&self, record: &LogRecord) {
		//Format the entry into an output string.
		let outStr = format!("{} {}: {}",
			record.location().module_path(),
			record.level(),
			record.args());
		//Actually write the log entry.
		try!(self.output.write(outStr));
	}
}