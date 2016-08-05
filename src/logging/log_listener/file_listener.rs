///Log listener that prints to a file.
extern crate log;

use ::logging::log_element::LogSeverity;
use ::logging::log_listener::interface::ListenerBase;
use std::fs::{File, OpenOptions};
use std::sync::Mutex;
use std::cell::RefCell;

pub type FileListener = ListenerBase<File>;

///Builder for FileListener instances.
#[derive(Debug)]
pub struct FileListenerBuilder {
	level: LogSeverity
}

impl FileListenerBuilder {
	pub fn new() -> FileListenerBuilder {
		FileListenerBuilder {
			level: LogSeverity::Info
		}
	}
	
	pub fn level(&mut self, val: LogSeverity) -> &mut FileListenerBuilder {
		self.level = val;
		self
	}

	///Builds a FileListener instance from the given settings.
	pub fn build(&self, path: &str) -> Result<FileListener, ()> {
		//Open the given file path.
		let file = OpenOptions::new()
					.write(true)
					.create(true)
					.truncate(false)
					.open(path);
		match file {
			Ok(opened_file) => {
				//Return the listener.
				return Ok(FileListener { output: Mutex::new(RefCell::new(opened_file)),
					level: self.level,
					output_ready: true });
			},
			_ => {
				return Err(());
			}
		}
	}
}