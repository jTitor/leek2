/**
Builds Logger instances given a minimum log level and
buffer size.
*/
use super::Logger;
use std::sync::Arc;
use time::Clock;

#[derive(Debug)]
pub struct LoggerBuilder {
	level: LogSeverity,
	buffer_size: usize,
	clock: Arc<Clock>
}

impl LoggerBuilder {
	pub fn new(clock: &Clock) -> LoggerBuilder {
		LoggerBuilder {
			level: LogSeverity::Info,
			buffer_size: 1024,
			clock: Arc::new(*clock)
		}
	}

	pub fn level(&mut self, val: LogSeverity) -> &mut LoggerBuilder {
		self.level = val;
		self
	}

	pub fn buffer_size(&mut self, val: usize) -> &mut LoggerBuilder {
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
				return Ok(
					Logger {
							level: self.level,
							out_file: opened_file,
							buffer: vec![0; self.buffer_size].into_boxed_slice(),
							buffer_head: 0,
							buffer_size: self.buffer_size,
							listeners: HashMap::new(),
							listener_next_id: 0,
							clock: self.clock
						}
					);
			},
			_ => {
				return Err(LogError::LoggerOutputNotReady);
			}
		}
	}
}