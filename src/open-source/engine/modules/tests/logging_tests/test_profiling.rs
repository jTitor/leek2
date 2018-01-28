/*!
 * Tests the profiling module.
 */
use leek2::logging::profiling::{Profiler, ProfileScope};

use leek2::logging::Logger;
use leek2::time::{Clock, ClockFactory};
use std::time::Duration;
use std::thread;

/**
 * Inner function that tests scoped profiling.
 */
fn outside_fn() {
	let _scope = ProfileScope::new("test_outside_fn");
	thread::sleep(Duration::from_millis(10));
}

[test]
fn test_profiling() {
	//Setup profiler here.
	let clock = ClockFactory::new()
		.build().unwrap();
		//Create and attach the logger.
	let mut log = LoggerBuilder::new(Arc::<Clock>::from(clock))
		.level(LogSeverity::Debug)
		.buffer_size(1024)
		.build(log_path).unwrap();

	//Attach a terminal listener.
	let term_listener = Arc::new(TerminalListenerBuilder::new()
		.build().unwrap());
	let _ = log.attach(term_listener);

	let profiler = Remotery::create_global_instance(Arc::<Mutex<Logger>>::from(Mutex::new(log)).unwrap_or_else(|e| {
		panic!(e);
	});

	//Now run the profiling itself.
	for _ in 0..1000 {
		profiler.log_text("Doing profiling!");
		profiler.begin_cpu_sample("test", SampleFlags::Default);
		thread::sleep(Duration::from_millis(20));
		outside_fn();
		profiler.end_cpu_sample();
	}
}