/*!
 * Tests the profiling module.
 */
use leek2::logging::profiling::{Profiler, ProfileScope};

use leek2::logging::{Logger, LoggerBuilder, LogSeverity, TerminalListenerBuilder};
use leek2::time::{Clock, ClockFactory};
use std::time::Duration;
use std::thread;
use std::sync::{Arc, RwLock};

/**
 * Inner function that tests scoped profiling.
 */
fn outside_fn() {
	let _scope = ProfileScope::new("test_outside_fn");
	thread::sleep(Duration::from_millis(10));
}

#[test]
fn test_profiling() {
	let log_path : &str = "./test.log";

	//Setup profiler here.
	let clock = ClockFactory::new()
		.build().unwrap();
		//Create and attach the logger.
	let mut log = LoggerBuilder::new(Arc::<Clock>::from(clock))
		.level(LogSeverity::Debug)
		.buffer_size(1024)
		.build(log_path).unwrap();
	let log_arc = Arc::<RwLock<&mut Logger>>::from(RwLock::new(&mut log));

	//Attach a terminal listener.
	let term_listener = Arc::new(TerminalListenerBuilder::new()
		.build().unwrap());
	{
		let _ = log_arc.write().unwrap().attach(term_listener);
	}

	let profiler = Profiler::create_global_instance(log_arc.clone()).unwrap_or_else(|e| {
		log_arc.write().unwrap().log_e("Failed to init profiler", "test_profiling");
		panic!(e);
	});

	{
		log_arc.write().unwrap().log_d("Beginning profiling test", "test_profiling");
	}

	//Now run the profiling itself.
	for _ in 0..1000 {
		profiler.begin_cpu_sample("test");
		thread::sleep(Duration::from_millis(1));
		outside_fn();
		profiler.end_cpu_sample();
	}

	{
		log_arc.write().unwrap().log_d("Profiling test complete", "test_profiling");
	}
}