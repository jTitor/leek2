///The logging subsystem.

//Use standard library's Log trait.
extern crate log;

use log::{LogRecord, LogLevel, LogMetadata};
use log_listener::interface::LogListen;

///Possible error codes for failures in log methods.
#[derive(PartialEq, Eq, Debug)]
pub enum LogError {
	ListenerNotReady,
	ListenerNotAttached,
}

///Handles logging requests.
#[derive(Debug)]
pub struct Logger {
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
		for listener in self.listeners.iter() {
			//Call the listener's log method.
			listener.on_log(record);
		}
	}

	///Links a LogListener to this Logger's buffer.
	///The LogListener will relay any log entries
	///recorded by the Logger.
	///Returns: Result::Ok if the LogListener was successfully attached,
	///Result::Err otherwise
	///(for instance, a listener's output file couldn't be opened).
	pub fn attach(&mut self, listener: &LogListen) {
		//Is this listener ready for attachment?
		//	If not, abort and return error.
		//Add this listener to the attached list.
		self.listeners.push(listener);
	}
	
	///Unlinks a specific LogListener from this Logger's buffer.
	pub fn detach(&mut self, listener: &LogListen) -> Result<(), LogError> {
		//Remove this listener from the attached list
		//if it was in the list in the first place.
		let listener_pos = self.listeners.binary_search(listener);
		match listener_pos {
			Ok(idx) => {
				self.listeners.remove(idx);
				return Ok();
			},
			_ => {
				return Err(LogError::ListenerNotAttached);
			}
		}
		unreachable!();
		Err(LogError::ListenerNotAttached);
	}
	
	///Unlinks *all* LogListeners from this Logger's buffer.
	pub fn detach_all(&mut self) {
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
				//If NOT:
				//Dump buffer to output file.
				//Dump line to output file.
				//Reset buffer head to the buffer's start.
			//Otherwise write this to the buffer, updating the buffer head.
		unimplemented!();
	}
}

pub struct LogBuilder {
	level: LogLevel,
	bufferSize: usize
}

impl LogBuilder {
	pub fn new() -> LogBuilder {
		LogBuilder {
			level: LogLevel::Info,
			bufferSize: 1024
		}
	}

	pub fn level(&mut self, val: LogLevel) -> &mut LogBuilder {
		self.level = val;
		self
	}

	pub fn bufferSize(&mut self, val: usize) -> &mut LogBuilder {
		self.bufferSize = val;
		self
	}

	pub fn build(&self) -> Logger {
		Logger {
			level: self.level,
			bufferHead: 0,
			bufferSize: self.bufferSize,
			listeners: vec![]
		}
	}
}