/*!
 * Defines the profiler struct
 * that manages all profiler logging state.
 */
use logging::profiling::ProfilerError;
use logging::Logger;
use std::sync::Arc;
use std::fmt;

use remotery::{Remotery, SampleFlags};

impl fmt::Debug for Remotery {
	//TODO
}

/**
 * The profiling manager.
 * 
 * Only one of these should be instantiated
 * at any given time, since the backend
 * is read via connecting to it as a
 * local HTTP service.
 */
#[derive(Debug)]
pub struct Profiler {
	profiler_impl: Remotery,
	logger: Arc<Logger>
}

impl Profiler {
	fn create_global_instance(logger: Arc<Logger>) -> Result<Profiler, ProfilerError> {
		unimplemented!();
		let profiler_instance = Remotery::create_global_instance()?;

		//Profiler should be static to this module.
		Ok(Profiler{profiler_impl: profiler_instance, logger: logger})
	}

	/**
	 * Logs raw text to the profiler.
	 */
	fn log_text(&self, text: &str) {
		self.logger.log_d(text, "profiler");
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