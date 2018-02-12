/*!
	The logging subsystem.
*/

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use logging::{LogSeverity, LogElement};
use super::log_error::LogError;
use super::log_listener::interface::LogListen;
use time::Clock;

/**
Handles logging requests.
*/
pub struct Logger {
	///The maximum filter level.
	///If an entry has a level higher than this,
	///it won't be logged to the buffer at all.
	pub level: LogSeverity,
	///The output file to write to.
	pub out_file: File,
	///Contains the last few logged strings.
	pub buffer: Box<[u8]>,
	///The current head of the log buffer.
	pub buffer_head: usize,
	///The length of the log buffer in characters.
	pub buffer_size: usize,
	///The LogListeners that are listening to this Logger.
	pub listeners: HashMap<u32, Arc<LogListen>>,
	pub listener_next_id: u32,
	pub clock: Arc<Clock>
}

impl fmt::Debug for Logger {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			"Logger {{ level: {}, buffer_size: {} }}",
			self.level, self.buffer_size)
	}
}

impl Logger {
	/**
	Sends the given log record to all LogListeners
	that are attached to this Logger.
	*/
	pub fn broadcast(&self, record: &LogElement) {
		//For all listeners:
		for (_, listener) in &self.listeners {
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

	/**
	Does actual work of printing a log entry.
	*/
	pub fn log(&mut self, text: &str, tag: &str, severity: LogSeverity) {
		//Don't bother if this is under our severity.
		if severity < self.level {
			return;
		}

		//Otherwise, broadcast it to any readers.
		let record = LogElement{text: text, tag: tag, severity: severity, time_stamp: self.clock.now_datetime()};
		self.broadcast(&record);
		//Can the buffer fit this line?
		if !self.can_fit(&record) {
			//If NOT, flush the buffer.
			self.flush();
			//Dump line to output file.
			self.dump(record.text.as_bytes());
		}
		//Otherwise we update the buffer.
		else {
			let record_len = record.text.len();
			self.buffer[self.buffer_head..self.buffer_head+record_len].clone_from_slice(&record.text.as_bytes());
			self.buffer_head += record_len;
		}
	}

	/**
	Logs a verbose message.
	*/
	pub fn log_v(&mut self, text: &str, tag: &str) {
		self.log(text, tag, LogSeverity::Verbose);
	}

	/**
	Logs a debug message.
	*/
	pub fn log_d(&mut self, text: &str, tag: &str) {
		self.log(text, tag, LogSeverity::Debug);
	}

	/**
	Logs a information message.
	*/
	pub fn log_i(&mut self, text: &str, tag: &str) {
		self.log(text, tag, LogSeverity::Info);
	}

	/**
	Logs a warning message.
	*/
	pub fn log_w(&mut self, text: &str, tag: &str) {
		self.log(text, tag, LogSeverity::Warning);
	}

	/**
	Logs an error message.
	*/
	pub fn log_e(&mut self, text: &str, tag: &str) {
		self.log(text, tag, LogSeverity::Error);
	}

	/**
	Links a LogListener to this Logger's buffer.
	The LogListener will relay any log entries
	recorded by the Logger.

	Returns: The listener's id in the logger if the LogListener was successfully attached, Result::Err otherwise
	(for instance, a listener's output file couldn't be opened).
	*/
	pub fn attach(&mut self, listener: Arc<LogListen>) -> Result<u32, LogError> {
		//Is this listener ready for attachment?
		//if !listener.output_ready {
		//	return Err(LogError::ListenerNotReady);
		//}

		//Otherwise add this listener to the attached list.
		self.listeners.insert(self.listener_next_id, listener);
		let result = self.listener_next_id;
		//Update the listener id.
		self.listener_next_id += 1;
		
		Ok(result)
	}
	
	/**
	Unlinks a specific LogListener from this Logger's buffer.
	*/
	pub fn detach(&mut self, listener_id: u32) -> Result<(), LogError> {
		//Remove this listener from the attached list
		//if it was in the list in the first place.
		if !self.listeners.contains_key(&listener_id) {
			return Err(LogError::ListenerNotAttached);
		}

		self.listeners.remove(&listener_id);
		Ok(())
	}
	
	/**
	Unlinks *all* LogListeners from this Logger's buffer.
	*/
	pub fn detach_all(&mut self) {
		//TODO: Should we tell listeners they've been detached?
		//Clear the attached list.
		self.listeners.clear();
	}

	/**
	Releases all resources used by this logger.
	*/
	pub fn shutdown(&mut self) {
		//Detach listeners, since we're going down.
		self.detach_all();
		//Can print a message that logger is closing...

		//Flush anything remaining in the buffer.
		self.flush();

		//...and once again we don't explicitly close files,
		//so we're done.
	}

	/**
	Determines if a given record's string data can fit
	entirely within the logger's buffer.
	*/
	pub fn can_fit(&self, record: &LogElement) -> bool {
		if self.buffer_head + record.text.to_string().len() > self.buffer_size {
			return false;
		}
		true
	}

	/**
	Writes given string buffer to the output file without
	posting it to the logger's buffer.
	*/
	pub fn dump(&mut self, buffer: &[u8]) {
		let result = self.out_file.write(buffer);
		match result {
			Ok(_) => {},
			_ => {
				//Couldn't write to file, report debug error
			}
		}
	}

	/**
	Flushes the buffer to the output file.
	*/
	pub fn flush(&mut self) {
		//Dump buffer to output file.
		//Remember - only write from the head to the end!
		let result = self.out_file.write(&self.buffer[..self.buffer_head]);
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

impl Drop for Logger {
	fn drop(&mut self) {
		self.shutdown();
	}
}