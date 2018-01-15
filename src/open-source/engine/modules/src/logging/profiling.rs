/*!
 * Wrapper for profiling library.
 */
use remotery::{Remotery, RemoteryScope, SampleFlags};
use remotery::error::RemoteryError;
use std::error::Error;
use std::fmt;

pub struct ProfileScope {
	scope_impl: RemoteryScope
}

impl ProfileScope {
	pub fn new(scope_name: &str) -> ProfileScope {
		ProfileScope {
			scope_impl: RemoteryScope::new(scope_name.clone(), SampleFlags::Default)
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub struct ProfilerError {
}

impl Error for ProfilerError {
	fn description(&self) -> &str {
		"unknown error in profiler backend"
	}

	// fn cause(&self) -> Option<&Error> {
	// 	//TODO: Not sure how we're going to go about error chaining. For now keep this in case
	// 	//we do want to do it.
	// 	match *self {
	// 		GameError::DeviceError{ref cause} => Some(cause),
	// 		GameError::WindowBuildFailed{ref cause} => Some(cause),
	// 		_ => None
	// 	}
	// }
}

impl fmt::Display for ProfilerError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut details: String = String::from(self.description());

		write!(f, "Profiler error: {}", details)
	}
}

impl From<RemoteryError> for ProfilerError {
	fn from(error: RemoteryError) -> Self {
		match error {
			_ => ProfilerError{}
		}
	}
}

pub struct Profiler {
	profiler_impl: Remotery
}

impl Profiler {
	fn create_global_instance() -> Result<Profiler, ProfilerError> {
		unimplemented!();
		let profiler_instance = Remotery::create_global_instance()?;

		//Profiler should be static to this module.
		Ok(Profiler{profiler_impl: profiler_instance})
	}

	/**
	 * Logs raw text to the profiler.
	 */
	fn log_text(&self, text: &str) {
		Remotery::log_text(text);
	}

	fn begin_cpu_sample(&self, text: &str) {
		Remotery::begin_cpu_sample(text, SampleFlags::Default);
	}

	pub fn end_cpu_sample(&self) {
		Remotery::end_cpu_sample();
	}
}