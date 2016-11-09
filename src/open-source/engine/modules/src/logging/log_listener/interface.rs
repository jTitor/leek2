///Interface for things that display/respond to log entries.
extern crate log;

use std::fmt;
use std::io::Write;
use std::sync::Mutex;
use std::cell::RefCell;
use ::logging::log_element::{LogSeverity, LogElement};
use ::logging::log_listener::listener_error::ListenerError;

///Base class for log listeners.
///Has a connection to some output stream
///and a maximum acceptable log level for filtering.
pub struct ListenerBase<T> where T: Write + Send {
	///The output file we're connected to.
	pub output: Mutex<RefCell<T>>,
	///The maximum log level to listen to.
	pub level: LogSeverity,
	///If true, output is connected and we can write
	///to it; otherwise the output is not connected
	///and attempts to write will probably cause a panic.
	pub output_ready: bool
}

impl<T> fmt::Debug for ListenerBase<T> where T: Write + Send {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
        	"ListenerBase {{ level: {} }}",
        	self.level)
    }
}

///Allows implementors to get a Logger's
///log entries.
pub trait LogListen : Send + Sync {
	///Called when a Logger has an entry for this listener to display.
	///This is only safe to call if output_ready == true.
	/// # Arguments
	/// * record: The newest log entry sent from the Logger.
	///Operates like println!() - the string should have a newline appended.
	fn on_log(&self, record: &LogElement) -> Result<(), ListenerError>;
}

//Most listeners just need to output to something that implements
//Write; how we get our Write reference is another story, hence the ListenerInit trait.
impl<T> LogListen for ListenerBase<T> where T: Write + Send {
	fn on_log(&self, record: &LogElement) -> Result<(), ListenerError> {
		//Format the entry into an output string.
		let out_str = format!("{} {}: {}\n",
			record.tag,
			record.severity,
			record.text);
		//Actually write the log entry.
		let output = try!(ListenerError::from_lock_result(self.output.lock()));
		try!(ListenerError::from_io_result(output.borrow_mut().write(out_str.as_bytes())));
		Ok(())
	}
}

impl<T> ListenerBase<T> where T: Write + Send {
	///Constructs the base elements of a listener.
	pub fn new(output: T, level: LogSeverity) -> ListenerBase<T> {
		ListenerBase {
			output: Mutex::new(RefCell::new(output)),
			level: level,
			output_ready: false
		}
	}
}