extern crate log;
extern crate leek2;
//Use macros from other crates.
#[macro_use]

use std::fs;
use std::sync::Arc;
use self::leek2::logging::logger::{Logger, LoggerBuilder};
use self::leek2::logging::log_element::LogSeverity;
use self::leek2::logging::log_listener::file_listener::{FileListener, FileListenerBuilder};
use self::leek2::logging::log_listener::terminal_listener::{TerminalListener, TerminalListenerBuilder};

#[test]
fn test_logging() {
	println!("Testing logging operations...");
	let log_path : &str = "test.log";
	let file_listener_path : &str = "fileListenerTest.log";
	{
		//Create and attach the logger.
		let mut log = LoggerBuilder::new()
			.level(LogSeverity::Debug)
			.buffer_size(1024)
			.build(log_path).unwrap();//LogHolder::init_global(LogLevel::Debug, 1024, "test.log");	

		//Attach a terminal listener.
		let term_listener = Arc::new(TerminalListenerBuilder::new()
			.build().unwrap());
		log.attach(term_listener);
		//Attach a file listener.
		let file_listener = Arc::new(FileListenerBuilder::new()
			.build(file_listener_path).unwrap());
		log.attach(file_listener);

		//Post all different log messages.
		let log_tag = "Test";
		log.log_v("Trace message; should not be visible", log_tag);
		log.log_d("Debug message.", log_tag);
		log.log_i("Info message.", log_tag);
		log.log_w("Warning message!", log_tag);
		log.log_w("Error message!", log_tag);

		//Detach listeners...
		log.detach_all();
	}
	println!("Logging test complete.");
	
	//Out of scope, so file references are closed.
	//Delete the log and listener files.
	//println!("Removing test log files...");
	//try!(fs::remove_file(log_path));
	//try!(fs::remove_file(file_listener_path));
	//println!("Test files removed.");
}