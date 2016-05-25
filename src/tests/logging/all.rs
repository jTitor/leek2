extern crate leek2;
//Use macros from other crates.
#[macro_use]
extern crate log;

use leek2::logging::logger::{Logger, LoggerBuilder};
use leek2::logging::log_element::LogSeverity;
use std::fs;

#[test]
fn test_logging() {
	println!("Testing logging operations...");
	let log_path : String = "test.log";
	let file_listener_path : String = "fileListenerTest.log";
	{
		//Create and attach the logger.
		let log = try!(LoggerBuilder.new()
			.level(LogSeverity::Debug)
			.buffer_size(1024)
			.build(log_path));//LogHolder::init_global(LogLevel::Debug, 1024, "test.log");	

		//Attach a terminal listener.
		let term_listener = try!(TerminalListenerBuilder.new()
			.build());
		log.attach(term_listener);
		//Attach a file listener.
		let file_listener = try!(FileListenerBuilder.new()
			.build(file_listener_path));
		log.attach(file_listener);

		//Post all different log messages.
		let log_tag = "Test";
		log.log_v("Trace message; should not be visible", log_tag);
		log.log_d("Debug message.", log_tag);
		log.log_i("Info message.", log_tag);
		log.log_w("Warning message!", log_tag);
		log.log_w("Error message!", log_tag);

		//Detach listeners...
		log_holder.detach_all();
	}
	println!("Logging test complete.");
	
	//Out of scope, so file references are closed.
	//Delete the log and listener files.
	//println!("Removing test log files...");
	//try!(fs::remove_file(log_path));
	//try!(fs::remove_file(file_listener_path));
	//println!("Test files removed.");
}