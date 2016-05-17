extern crate leek2;
//Use macros from other crates.
#[macro_use]
extern crate log;

use leek2::logging::log_holder::LogHolder;
use log::LogLevel;

#[test]
fn test_logging() {
	//Create and attach the logger.
	let log_holder = try!(LoggerBuilder.new()
		.level(LogLevel::Debug)
		.buffer_size(1024)
		.build("test.log"));//LogHolder::init_global(LogLevel::Debug, 1024, "test.log");	

	//Attach a terminal listener.
	let term_listener = try!(TerminalListenerBuilder.new()
		.build());
	//Attach a file listener.
	let file_listener = try!(FileListenerBuilder.new()
		.build("fileListenerTest.log"));

	//Post all different log messages.
	trace!("Trace message; should not be visible");
	debug!("Debug message.");
	info!("Info message.");
	warn!("Warning message!");
	error!("Error message!");

	//Detach listeners...
	log_holder.detach_all();
	//Now we can close our listeners.
	//Close the logger.

	//Delete the log and listener files.
}