/*!
 * Defines scoped profiling structs.
 */

use remotery::{Remotery, RemoteryScope, SampleFlags};

/**
 * A scoped profiling section. When this struct is created
 * with new(), it starts a profiling section as if
 * Profiler::begin_cpu_sample() were called, and when the
 * struct is dropped, it ends the section as if
 * Profiler::end_cpu_sample() were called.
 */
#[derive(Debug)]	//Don't think it makes sense for this to be Copy/Clone
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