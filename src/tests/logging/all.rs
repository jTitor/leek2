extern crate leek2;
//Use macros from other crates.
#[macro_use]
extern crate log;

use leek2::logging::log_holder::LogHolder;
use log::LogLevel;

#[test]
fn test_logging() {
	//Create and attach the logger.
	let log_holder = LogHolder::init_global(LogLevel::Debug, 1024, "test.log");	

	//Attach a terminal listener.
	//Attach a file listener.


	//Post all different log messages.
	trace!("Trace message; should not be visible");
	debug!("Debug message.");
	info!("Info message.");
	warn!("Warning message!");
	error!("Error message!");

	//Close our listeners.
	//Close the logger.
	LogHolder::shutdown_global();

	//Delete the log and listener files.
}