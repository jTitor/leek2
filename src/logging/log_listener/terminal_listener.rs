///Log listener that prints to a terminal's output.
extern crate log;

use logging::log_listener::interface::ListenerBase;
use std::io;

type TerminalListener = ListenerBase<io::Stdout>;

impl ListenerInit for TerminalListener {
	fn shutdown(&self) {
		//Do nothing.
	}
}

struct TerminalListenerBuilder {
	level: LogLevel
}

impl TerminalListenerBuilder {
	fn new() -> TerminalListenerBuilder {
		TerminalListenerBuilder {
			level: LogLevel::Info
		}
	}
	
	fn level(&mut self, val: LogLevel) -> &mut TerminalListenerBuilder {
		self.level = val;
		self
	}
	
	fn build(&self) -> Result<TerminalListener, ()> {
		//Return the listener.
		Ok(TerminalListener { output: io::stdout(), level: self.level, output_ready: true })
	}
}