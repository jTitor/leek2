///Log listener that prints to a terminal's output.
extern crate log;

use self::log::LogLevel;
use ::logging::log_listener::interface::{ListenerBase, ListenerInit};
use std::io;
use std::fmt;
use std::sync::Mutex;
use std::cell::RefCell;

pub type TerminalListener = ListenerBase<io::Stdout>;

impl ListenerInit for TerminalListener {
	fn shutdown(&self) {
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
		Ok(TerminalListener { output: Mutex::new(RefCell::new(io::stdout())),
			level: self.level,
			output_ready: true })
	}
}