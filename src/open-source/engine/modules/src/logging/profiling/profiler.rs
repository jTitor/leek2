/*!
 * Defines the profiler struct
 * that manages all profiler logging state.
 */
use logging::profiling::ProfilerError;
use logging::Logger;
use std::sync::{Arc, Mutex};
use std::fmt;

use remotery::{Remotery, SampleFlags};

/**
 * The profiling manager.
 * 
 * Only one of these should be instantiated
 * at any given time, since the backend
 * is read via connecting to it as a
 * local HTTP service.
 */
pub struct Profiler {
	profiler_impl: Remotery,
	logger: Arc<Mutex<Logger>>
}

impl Profiler {
	fn create_global_instance(logger: Arc<Mutex<Logger>>) -> Result<Profiler, ProfilerError> {
		let profiler_instance = Remotery::create_global_instance()?;

		//Profiler should be static to this module.
		Ok(Profiler{profiler_impl: profiler_instance, logger: logger})
	}

	/**
	 * Logs raw text to the profiler.
	 */
	fn log_text(&mut self, text: &str) {
		self.logger.lock().unwrap().log_d(text, "profiler");
		Remotery::log_text(text);
	}

	/**
	 * Manually starts a profiling section;
	 * to end the section, call end_cpu_sample().
	 *
	 * Parameters:
	 *  * text: A readable description of the profiling section.
	 */
	fn begin_cpu_sample(&self, text: &str) {
		Remotery::begin_cpu_sample(text, SampleFlags::Default);
	}

	/**
	 * Ends a profiling section started by
	 * begin_cpu_sample().
	 */
	pub fn end_cpu_sample(&self) {
		Remotery::end_cpu_sample();
	}
}

impl fmt::Debug for Profiler {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Device {{ logger: {:?} }}", self.logger)
	}
}