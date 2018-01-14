/*!
 * Wrapper for profiling library.
 */
use remotery::{Remotery, RemoteryScope, SampleFlags, RemoteryError};

pub struct ProfileScope {
	scope_impl: RemoteryScope
}

impl ProfileScope {
	pub fn new(scope_name: &str) -> ProfileScope {
		ProfileScope {
			scope_impl: RemoteryScope::new(scope_name.clone, SampleFlags::Default)
		}
	}
}

pub struct ProfilerError {
}

impl Error for ProfilerError {
	unimplemented!();
}

impl From<RemoteryError> for ProfilerError {
	unimplemented!();
}

pub struct Profiler {
	profiler_impl: Remotery
}

impl Profiler {
	pub fn create_global_instance() -> Profiler {
		let profiler_instance = Remotery::create_global_instance()?

		//Profiler should be static to this module.
		unimplemented!()
		Profiler{profiler_impl: Remotery{}}
	}

	/**
	 * Logs raw text to the profiler.
	 */
	pub fn log_text(&self, text: &str) {
		Remotery::log_text(text);
	}

	pub fn begin_cpu_sample(&self, text: &str) {
		Remotery::begin_cpu_sample(text, SampleFlags::Default);
	}

	pub fn end_cpu_sample() {
		Remotery::end_cpu_sample();
	}
}