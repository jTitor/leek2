///Log listener that prints to a file.
extern crate log;

use self::log::LogLevel;
use ::logging::log_listener::interface::{ListenerBase, ListenerInit};
use std::fs::{File, OpenOptions};
use std::sync::Mutex;
use std::cell::RefCell;

pub type FileListener = ListenerBase<File>;

impl ListenerInit for FileListener {
	fn shutdown(&self) {
		//...The file closes itself???
		//No need to do anything, apparently.
	}
}

///Builder for FileListener instances.
#[derive(Debug)]
pub struct FileListenerBuilder {
	level: LogLevel
}

impl FileListenerBuilder {
	pub fn new() -> FileListenerBuilder {
		FileListenerBuilder {
			level: LogLevel::Info
		}
	}
	
	pub fn level(&mut self, val: LogLevel) -> &mut FileListenerBuilder {
		self.level = val;
		self
	}

	///Builds a FileListener instance from the given settings.
	pub fn build(&self, path: String) -> Result<FileListener, ()> {
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