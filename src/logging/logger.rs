///The logging subsystem.

//Use standard library's Log trait.
extern crate log;

use log::{LogRecord, LogLevel, LogMetadata};
use log_listener::interface::LogListen;

///Possible error codes for failures in log methods.
#[derive(PartialEq, Eq, Debug)]
enum LogError {
	ListenerNotReady,
	ListenerNotAttached,
}

///Handles logging requests.
#[derive(Debug)]
struct Logger {
	///The maximum filter level.
	///If an entry has a level higher than this,
	///it won't be logged to the buffer at all.
	level: LogLevel,
	///Contains the last few logged strings.
	//buffer:
	///The current head of the log buffer.
	bufferHead: usize,
	///The length of the log buffer in characters.
	bufferSize: usize,
	///The LogListeners that are listening to this Logger.
	listeners: Vec<&LogListen>
}

//TODO: It's more idiomatic to return a Result struct
//when you're expecting to return error codes;
//refactor methods to return Results instead.
impl Logger {
	///Sends the given log record to all LogListeners
	///that are attached to this Logger.
	fn broadcast(&self, record: &LogRecord) {
		//For all listeners:
			//Call the listener's log method.
		unimplemented!();
	}
	
	///Links a LogListener to this Logger's buffer.
	///The LogListener will relay any log entries
	///recorded by the Logger.
	///Returns: Result::Ok if the LogListener was successfully attached,
	///Result::Err otherwise
	///(for instance, a listener's output file couldn't be opened).
	fn attach(&self, listener: &LogListen) -> Result<(), LogError> {
		//Is this listener ready for attachment?
		//	If not, abort and return error.
		//Add this listener to the attached list.
		unimplemented!();
	}
	
	///Unlinks a specific LogListener from this Logger's buffer.
	fn detach(&self, listener: &LogListen) -> Result<(), LogError> {
		//Remove this listener from the attached list
		//if it was in the list in the first place.
		unimplemented!();
	}
	
	///Unlinks *all* LogListeners from this Logger's buffer.
	fn detach_all(&self) {
		//Clear the attached list.
		self.listeners.clear();
	}
}

impl log::Log for Logger {
	///Determines if the given log entry should be logged. 
	fn enabled(&self, metadata: &LogMetadata) -> bool {
		//Is this entry's level below our maximum filter level?
		metadata.level() <= self.level
	}
	
	///Does actual work of printing a log entry.
	fn log(&self, record: &LogRecord) {
		//Are we supposed to log this entry?
			//If so, broadcast it to any readers.
			//Can the buffer fit this line?
				//If NOT, dump to output file.
				//Reset buffer head to the buffer's start.
			//Write this to the buffer, updating the buffer head.
		unimplemented!();
	}
}