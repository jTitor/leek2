//Module definition for logging.

//Log error enum.
pub mod log_error;

//The logging subsystem.
pub mod logger;

//ontainer class that links logger to log::Log.
pub mod log_holder;

//Interface for things that display/respond to log entries.
pub mod log_listener;