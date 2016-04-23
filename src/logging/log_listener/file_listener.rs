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
	file_path: String,
	level: LogLevel
}

impl FileListenerBuilder {
	pub fn new() -> FileListenerBuilder {
		FileListenerBuilder {
			file_path: String::from(""),
			level: LogLevel::Info
		}
	}
	
	pub fn file_path(&mut self, val: String) -> &mut FileListenerBuilder {
		self.file_path = val;
		self
	}
	
	pub fn level(&mut self, val: LogLevel) -> &mut FileListenerBuilder {
		self.level = val;
		self
	}

	///Builds a FileListener instance from the given settings.
	pub fn build(&self) -> Result<FileListener, ()> {
		//Open the given file path.
		let file = OpenOptions::new()
					.write(true)
					.create(true)
					.truncate(false)
					.open(&self.file_path);
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
		unreachable!();
		Err(())
	}
}