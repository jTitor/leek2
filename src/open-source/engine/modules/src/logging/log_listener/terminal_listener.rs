/*!
	Log listener that prints to a terminal's output.
*/
extern crate log;

use std::cell::RefCell;
use std::io;
use std::sync::RwLock;
use super::interface::ListenerBase;
use logging::LogSeverity;

pub type TerminalListener = ListenerBase<io::Stdout>;

/**
Builder for TerminalListener instances.
*/
#[derive(Debug)]
pub struct TerminalListenerBuilder {
	level: LogSeverity
}

impl TerminalListenerBuilder {
	pub fn new() -> TerminalListenerBuilder {
		TerminalListenerBuilder {
			level: LogSeverity::Info
		}
	}

	///Sets verbosity level.
	pub fn level(&mut self, val: LogSeverity) -> &mut TerminalListenerBuilder {
		self.level = val;
		self
	}

	///Builds a TerminalListener instance from the given settings.
	pub fn build(&self) -> Result<TerminalListener, ()> {
		//Return the listener.
		Ok(TerminalListener { output: RwLock::new(RefCell::new(io::stdout())),
			level: self.level,
			output_ready: true,
			show_severity: true,
			show_tag: true,
			show_timestamp: true })
	}
}