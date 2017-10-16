//Module definition for logging.

//A single logging element.
pub mod log_severity;
pub mod log_element;
pub use log_severity::LogSeverity;
pub use log_element::LogElement;


//Log error enum.
pub mod log_error;
pub use log_error::LogError;

//The logging subsystem.
pub mod logger;
pub use logger::Logger;
pub mod log_builder;
pub use log_builder::LoggerBuilder;

//Interface for things that display/respond to log entries.
pub mod log_listener;
pub use log_listener::interface::LogListen;
pub use log_listener::listener_error::ListenerError;
pub use log_listener::terminal_listener::{TerminalListener, TerminalListenerBuilder};
pub use log_listener::file_listener::{FileListener, FileListenerBuilder};