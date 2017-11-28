use std::sync::Arc;
use std::convert::From;
use leek2::logging::LoggerBuilder;
use leek2::logging::LogSeverity;
use leek2::logging::{FileListenerBuilder, TerminalListenerBuilder};
use leek2::time::{ClockFactory, Clock};

/**
 * Currently just tests that logging methods
 * don't panic when used; this doesn't
 * currently validate screen output.
 */
#[test]
fn test_logging() {
	println!("Testing logging operations...");
	let log_path : &str = "./test.log";
	let file_listener_path : &str = "./fileListenerTest.log";
	{
		let clock = ClockFactory::new()
		.build().unwrap();
		//Create and attach the logger.
		let mut log = LoggerBuilder::new(Arc::<Clock>::from(clock))
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