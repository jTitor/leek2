//Module definition for logging.

//A single logging element.
pub mod log_severity;
pub mod log_element;
pub use self::log_severity::LogSeverity;
pub use self::log_element::LogElement;


//Log error enum.
pub mod log_error;
pub use self::log_error::LogError;

//The logging subsystem.
pub mod logger;
pub use self::logger::Logger;
pub mod log_builder;
pub use self::log_builder::LoggerBuilder;

//Interface for things that display/respond to log entries.
pub mod log_listener;
pub use self::log_listener::interface::LogListen;
pub use self::log_listener::listener_error::ListenerError;
pub use self::log_listener::terminal_listener::{TerminalListener, TerminalListenerBuilder};
pub use self::log_listener::file_listener::{FileListener, FileListenerBuilder};