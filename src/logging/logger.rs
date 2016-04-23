///The logging subsystem.

//Use standard library's Log trait.
extern crate log;

use self::log::{LogRecord, LogLevel, LogMetadata};
use ::logging::log_listener::interface::LogListen;
use std::fs::File;
use std::fs::OpenOptions;
use std::fmt;

///Possible error codes for failures in log methods.
#[derive(PartialEq, Eq, Debug)]
pub enum LogError {
	LoggerOutputNotReady,
	ListenerNotReady,
	ListenerNotAttached,
}

///Handles logging requests.
pub struct Logger<'a> {
	///The maximum filter level.
	///If an entry has a level higher than this,
	///it won't be logged to the buffer at all.
	pub level: LogLevel,
	///The output file to write to.
	out_file: File,
	///Contains the last few logged strings.
	buffer: Box<[u8]>,
	///The current head of the log buffer.
	buffer_head: usize,
	///The length of the log buffer in characters.
	pub buffer_size: usize,
	///The LogListeners that are listening to this Logger.
	listeners: Vec<&'a LogListen>
}

impl<'a> fmt::Debug for Logger<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
        	"Logger {{ level: {}, buffer_size: {} }}",
        	self.level, self.buffer_size)
    }
}

//TODO: It's more idiomatic to return a Result struct
//when you're expecting to return error codes;
//refactor methods to return Results instead.
impl<'a> Logger<'a> {
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
	pub fn attach(&mut self, listener: &'a LogListen) -> Result<(), LogError> {
		//Is this listener ready for attachment?
		//	If not, abort and return error.
		//Add this listener to the attached list.
		self.listeners.push(listener);
		Ok(())
	}
	
	///Unlinks a specific LogListener from this Logger's buffer.
	pub fn detach(&mut self, listener: &LogListen) -> Result<(), LogError> {
		//Remove this listener from the attached list
		//if it was in the list in the first place.
		let listener_pos = self.listeners.binary_search_by(|probe| (probe as *const &LogListen).cmp(&(&listener as *const &LogListen)));
		match listener_pos {
			Ok(idx) => {
				self.listeners.remove(idx);
				return Ok(());
			},
			_ => {
				return Err(LogError::ListenerNotAttached);
			}
		}
	}
	
	///Unlinks *all* LogListeners from this Logger's buffer.
	pub fn detach_all(&mut self) {
		//Clear the attached list.
		self.listeners.clear();
	}

	///Flushes the buffer to the output file.
	pub fn flush(&mut self) {
		//TODO
	}
}

impl<'a> log::Log for Logger<'a> {
	///Determines if the given log entry should be logged. 
	fn enabled(&self, metadata: &LogMetadata) -> bool {
		//Is this entry's level below our maximum filter level?
		metadata.level() <= self.level
	}
	
	///Does actual work of printing a log entry.
	fn log(&self, record: &LogRecord) {
		//Are we supposed to log this entry?
		if self.enabled(record.metadata()) {
			//If so, broadcast it to any readers.
			for listener in self.listeners.iter() {
				listener.on_log(record);
			}
			//Can the buffer fit this line?
				//If NOT:
				//Dump buffer to output file.
				//Dump line to output file.
				//Reset buffer head to the buffer's start.
			//Otherwise write this to the buffer, updating the buffer head.
			unimplemented!();
		}
	}

	//TODO: Add shutdown code that flushes buffer.
}

pub struct LogBuilder {
	level: LogLevel,
	buffer_size: usize,
}

impl LogBuilder {
	pub fn new() -> LogBuilder {
		LogBuilder {
			level: LogLevel::Info,
			buffer_size: 1024
		}
	}

	pub fn level(&mut self, val: LogLevel) -> &mut LogBuilder {
		self.level = val;
		self
	}

	pub fn buffer_size(&mut self, val: usize) -> &mut LogBuilder {
		//Buffers can't be zero-sized, but anything else goes.
		if val <= 0 {
			return self;
		}
		self.buffer_size = val;
		self
	}

	pub fn build(&self, out_file_path: &str) -> Result<Logger, LogError> {
		//Connect to our output file.
		//If that failed, abort here.
		let file = OpenOptions::new()
					.read(true)
					.write(true)
					.append(true)
					.create(true)
					.open(out_file_path);
		match file {
			Ok(opened_file) => {
				//Otherwise, construct our Logger now.
				return Ok(Logger {
					level: self.level,
					out_file: opened_file,
					buffer: vec![0; self.buffer_size].into_boxed_slice(),
					buffer_head: 0,
					buffer_size: self.buffer_size,
					listeners: vec![]
				});
			},
			_ => {
				return Err(LogError::LoggerOutputNotReady);
			}
		}
	}
}