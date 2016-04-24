///Acts as bridge between actual logging module and log::Log.

//Use standard library's Log crate.
extern crate log;

use std::sync::{Mutex, Arc};
use std::cell::RefCell;
//use std::rc::Rc;
use self::log::{LogRecord, LogLevel, LogMetadata, LogLevelFilter, MaxLogLevelFilter};
use ::logging::logger::{Logger, LoggerBuilder};
use ::logging::log_error::LogError;

pub struct LogHolder {
	pub logger: Mutex<RefCell<Logger>>,
	log_filter: Arc<MaxLogLevelFilter>
}

impl log::Log for LogHolder {
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

impl LogHolder {
	pub fn get_level(&self) -> Result<LogLevel, LogError> {
		let lock = try!(LogError::from_lock_result(self.logger.lock()));
		let logger = lock.borrow();
		Ok(logger.level)
	}

	pub fn set_level(&self, level: LogLevel) -> Result<(), LogError> {
		//First set the logger's level.
		let lock = try!(LogError::from_lock_result(self.logger.lock()));
		let mut logger = lock.borrow_mut();
		logger.level = level;
		//Now set the log filter's level with it.
		self.log_filter.set(LogHolder::level_to_filter(level));
		Ok(())
	}

	fn level_to_filter(level: LogLevel) -> LogLevelFilter {
		match level {
			LogLevel::Trace => { return LogLevelFilter::Trace; },
			LogLevel::Debug => { return LogLevelFilter::Debug; },
			LogLevel::Warn => { return LogLevelFilter::Warn; },
			LogLevel::Error => { return LogLevelFilter::Error; },
			LogLevel::Info => { return LogLevelFilter::Info; },
		}
	}

	///Creates a global LogHolder instance and attaches it to log::Log.
	fn init_global(level: LogLevel, buffer_size: usize, out_file_path: &str) -> Result<(), LogError> {
		//First try to build the logger.
		let build = try!(LoggerBuilder::new()
				.level(level)
				.buffer_size(buffer_size)
				.build(out_file_path));
		//Now actually attach the logger.
		log::set_logger(|max_log_level| {
			max_log_level.set(LogHolder::level_to_filter(level));
			let holder_instance = Box::new(LogHolder{
				logger: Mutex::new(RefCell::new(build)),
				log_filter: Arc::new(max_log_level)
			});
			holder_instance
		});
		Ok(())
	}

	///Detaches the global instance from log::Log.
	fn shutdown_global() {
		let result = log::shutdown_logger();
		match result {
			Ok(_) => {},
			_ => {
				//Error ocurred during shutdown!
			}
		}
	}
}