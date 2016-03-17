///Interface for things that display/respond to log entries.
extern crate log;

use log::{LogRecord, LogLevel, LogMetadata};
use std::io::Write;

///Base class for log listeners.
///Has a connection to some output stream
///and a maximum acceptable log level for filtering.
struct ListenerBase<T: Write> {
	///The output file we're connected to.
	output: &T,
	///The maximum log level to listen to.
	level: LogLevel,
	///If true, output is connected and we can write
	///to it; otherwise the output is not connected
	///and attempts to write will probably cause a panic.
	output_ready: bool
}

///Allows implementors to get a Logger's
///log entries.
trait LogListen {
	///Called when a Logger has an entry for this listener to display.
	///This is only safe to call if output_ready == true.
	/// # Arguments
	///
	/// * record: The newest log entry sent from the Logger.
	fn on_log(&self, record: &LogRecord);
}

///Allows implementors to initialize and
///release logging resources before/after logging activities.
trait ListenerInit : LogListen {
	///Called to shut down the listener.
	fn shutdown(&self);
}

//Most listeners just need to output to something that implements
//Write; how we get our Write reference is another story, hence the ListenerInit trait.
impl<T> LogListen for ListenerBase<T> {
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

impl<T> ListenerBase<T> {
	fn new(output: &T, level: LogLevel) -> ListenerBase<T> {
		ListenerBase {
			output: output,
			level: level,
			output_ready: false
		}
	}
}