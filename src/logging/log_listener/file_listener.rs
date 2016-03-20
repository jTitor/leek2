///Log listener that prints to a file.
extern crate log;

use log::LogLevel;
use logging::log_listener::LogListener;
use std::fs::{File, OpenOptions};

pub type FileListener = ListenerBase<File>;

pub impl ListenerInit for FileListener {
	fn shutdown(&self) {
		//...The file closes itself???
		//No need to do anything, apparently.
	}
}

///Builder for FileListener instances.
#[derive(Debug)]
pub struct FileListenerBuilder {
	file_path: &str,
	level: LogLevel
}

impl FileListenerBuilder {
	pub fn new() -> FileListenerBuilder {
		FileListenerBuilder {
			file_path:"",
			level: LogLevel::Info
		}
	}
	
	pub fn file_path(&mut self, val: &str) -> &mut FileListenerBuilder {
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
		let file = try!(OpenOptions::new()
					.write(true)
					.create(true)
					.truncate(false)
					.open(self.file_path));
		//Return the listener.
		Ok(FileListener { output: file, level: self.level, output_ready: true })
	}
}