///Log listener that prints to a terminal's output.
extern crate log;

use self::log::LogLevel;
use ::logging::log_listener::interface::{ListenerBase, ListenerInit};
use std::io;

pub type TerminalListener = ListenerBase<io::Stdout>;

impl ListenerInit for TerminalListener {
	pub fn shutdown(&self) {
		//Do nothing.
	}
}

///Builder for TerminalListener instances.
#[derive(Debug)]
pub struct TerminalListenerBuilder {
	level: LogLevel
}

impl TerminalListenerBuilder {
	pub fn new() -> TerminalListenerBuilder {
		TerminalListenerBuilder {
			level: LogLevel::Info
		}
	}

	//Sets verbosity level.
	pub fn level(&mut self, val: LogLevel) -> &mut TerminalListenerBuilder {
		self.level = val;
		self
	}

	///Builds a TerminalListener instance from the given settings.
	pub fn build(&self) -> Result<TerminalListener, ()> {
		//Return the listener.
		Ok(TerminalListener { output: io::stdout(), level: self.level, output_ready: true })
	}
}