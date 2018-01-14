/*!
 * Tests the profiling module.
 */
use leek2::logging::profiling::{Profiler, ProfileScope};

/*
 *use std::time::Duration;
use std::thread;

fn some_function() {
    let _scope = RemoteryScope::new("some_function", SampleFlags::Default);
    thread::sleep(Duration::from_millis(10));
}

fn main() {
    let _remotery = Remotery::create_global_instance().unwrap_or_else(|e| {
    	panic!(e);
	});

    for _ in 0..1000 {
        Remotery::log_text("Doing profiling!");
        Remotery::begin_cpu_sample("test", SampleFlags::Default);
        thread::sleep(Duration::from_millis(20));
        some_function();
        Remotery::end_cpu_sample();
    }
} 
 */

fn outside_fn() {
	let _scope = ProfileScope::new("test_outside_fn");
	thread::sleep(Duration::from_millis(10));
}

[test]
fn test_profiling() {
	//Setup profiler here.
	let _remotery = Remotery::create_global_instance()
	//Now run the profiling itself.
}