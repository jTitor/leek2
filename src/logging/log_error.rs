///Possible error codes for failures in log methods.
use std::sync;
use std::io;

#[derive(PartialEq, Eq, Debug)]
pub enum LogError {
	LoggerMutexNotReady,
	LoggerOutputNotReady,
	ListenerNotReady,
	ListenerNotAttached,
}

impl LogError {
	pub fn from_lock_result<T>(result: sync::LockResult<T>) -> Result<T, LogError> {
		match result {
			Ok(guard) => { Ok(guard) },
			_ => { Err(LogError::LoggerMutexNotReady) }
		}
	}

	pub fn from_io_result<T>(result: io::Result<T>) -> Result<T, LogError> {
		match result {
			Ok(value) => { Ok(value) },
			_ => { Err(LogError::LoggerOutputNotReady) }
		}	
	}
}