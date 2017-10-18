use std::sync::Arc;
use self::leek2::logging::logger::{LoggerBuilder};
use self::leek2::logging::log_element::LogSeverity;
use self::leek2::logging::log_listener::file_listener::{FileListenerBuilder};
use self::leek2::logging::log_listener::terminal_listener::{TerminalListenerBuilder};

#[test]
fn test_logging() {
	println!("Testing logging operations...");
	let log_path : &str = "./test.log";
	let file_listener_path : &str = "./fileListenerTest.log";
	{
		//Create and attach the logger.
		let mut log = LoggerBuilder::new()
			.level(LogSeverity::Debug)
			.buffer_size(1024)
			.build(log_path).unwrap();

		//Attach a terminal listener.
		let term_listener = Arc::new(TerminalListenerBuilder::new()
			.build().unwrap());
		let _ = log.attach(term_listener);
		//Attach a file listener.
		//TODO: make severity levels an option on listeners instead of on the logger?
		let file_listener = Arc::new(FileListenerBuilder::new()
			.build(file_listener_path).unwrap());
		let _ = log.attach(file_listener);

		//Post all different log messages.
		let log_tag = "Test";
		log.log_v("Trace message; should not be visible.\n", log_tag);
		log.log_d("Debug message.\n", log_tag);
		log.log_i("Info message.\n", log_tag);
		log.log_w("Warning message!\n", log_tag);
		log.log_e("Error message!\n", log_tag);

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