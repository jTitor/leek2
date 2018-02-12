/*!
 * Defines scoped profiling structs.
 */

use remotery::{RemoteryScope, SampleFlags};
use std::fmt;

/**
 * A scoped profiling section. When this struct is created
 * with new(), it starts a profiling section as if
 * Profiler::begin_cpu_sample() were called, and when the
 * struct is dropped, it ends the section as if
 * Profiler::end_cpu_sample() were called.
 */
//Don't think it makes sense for this to be Copy/Clone
pub struct ProfileScope {
	_scope_impl: RemoteryScope
}

impl fmt::Debug for ProfileScope {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "ProfileScope {{ }}")//, self.scope_impl)
	}
}

impl ProfileScope {
	pub fn new(scope_name: &str) -> ProfileScope {
		ProfileScope {
			_scope_impl: RemoteryScope::new(scope_name.clone(), SampleFlags::Default)
		}
	}
}