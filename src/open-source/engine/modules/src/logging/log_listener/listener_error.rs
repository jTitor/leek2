/*!
	Definition of errors during listener usage.
*/
use std::sync;
use std::io;

#[derive(Debug)]
pub enum ListenerError {
	OutputLockNotReady,
	OutputNotReadyForWrite
}

impl ListenerError {
	pub fn from_lock_result<T>(result: sync::LockResult<T>) -> Result<T, ListenerError> {
		match result {
			Ok(guard) => { Ok(guard) },
			_ => { Err(ListenerError::OutputLockNotReady) }
		}
	}

	pub fn from_io_result<T>(result: io::Result<T>) -> Result<T, ListenerError> {
		match result {
			Ok(value) => { Ok(value) },
			_ => { Err(ListenerError::OutputNotReadyForWrite) }
		}	
	}
}