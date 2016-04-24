///The logging subsystem.

//Use standard library's Log trait.
extern crate log;

use self::log::{LogRecord, LogLevel, LogMetadata};
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;
use std::fmt;
use std::sync::Mutex;
use std::cell::RefCell;
use ::logging::log_listener::interface::LogListen;
use ::logging::log_error::LogError;

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
			let result = listener.on_log(record);
			match result {
				Ok(_) => {},
				_ => {
					//An error ocurred, we could notify here in debug mode.
				}
			}
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

	pub fn can_fit(&self, record: &LogRecord) -> bool {
		//TODO
		if record.args().to_string().len() >= self.buffer_size {
			return false;
		}
		false
	}

	///Writes given string buffer to the output file without posting it to the buffer.
	pub fn dump(&mut self, buffer: &[u8]) {
		let result = self.out_file.write(buffer);
		match result {
			Ok(_) => {},
			_ => {
				//Couldn't write to file, report debug error
			}
		}
	}

	///Flushes the buffer to the output file.
	pub fn flush(&mut self) {
		//Dump buffer to output file.
		//self.dump(self.buffer.as_ref());
		let result = self.out_file.write(&self.buffer);
		match result {
			Ok(_) => {},
			_ => {
				//Couldn't write to file, report debug error
			}
		}

		//Reset buffer head to the buffer's start.
		self.buffer_head = 0;
	}
}

pub struct LogHolder<'a> {
	pub logger: Mutex<RefCell<Logger<'a>>>
}

impl<'a> log::Log for LogHolder<'a> {
	///Determines if the given log entry should be logged. 
	fn enabled(&self, metadata: &LogMetadata) -> bool {
		let result = self.logger.lock();
		match result {
			Ok(cell) => {
				//Is this entry's level below our maximum filter level?
				return metadata.level() <= cell.borrow().level;
			},
			_ => {
				//Couldn't get logger, assume we can't log.
				return false;
			}
		}
	}
	
	///Does actual work of printing a log entry.
	fn log(&self, record: &LogRecord) {
		let result = self.logger.lock();
		match result {
			Ok(cell) => {
				//Are we supposed to log this entry?
				if self.enabled(record.metadata()) {
					let mut logger = cell.borrow_mut();
					//If so, broadcast it to any readers.
					logger.broadcast(record);
					//Can the buffer fit this line?
					if !logger.can_fit(record) {
						//If NOT, flush the buffer.
						logger.flush();
						//Dump line to output file.
						logger.dump(record.args().to_string().as_bytes());
					}
					//Otherwise write this to the buffer, updating the buffer head.
					unimplemented!();
				}
			},
			_ => {
				//Couldn't get the logger.
				return;
			}
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

	pub fn build(&self, out_file_path: &str) -> Result<LogHolder, LogError> {
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
				return Ok(LogHolder{
						logger: Mutex::new(RefCell::new(Logger {
						level: self.level,
						out_file: opened_file,
						buffer: vec![0; self.buffer_size].into_boxed_slice(),
						buffer_head: 0,
						buffer_size: self.buffer_size,
						listeners: vec![]
					}))
				});
			},
			_ => {
				return Err(LogError::LoggerOutputNotReady);
			}
		}
	}
}