/*!
	Entry point for testing profiling system.
	This builds the engine as a library,
	then uses the library in an executable.
*/
extern crate leek2;
use leek2::GameBuilder;
use leek2::logging::profiling::{Profiler, ProfileScope};
use leek2::logging::{Logger, LoggerBuilder, LogSeverity, TerminalListenerBuilder};
use leek2::time::{Clock, ClockFactory};
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
	let log_path : &str = "./test.log";
	//Setup profiler here.
	let clock = ClockFactory::new()
		.build().unwrap();

	//Create and attach the logger.
	let mut log = LoggerBuilder::new(Arc::<Clock>::from(clock))
		.level(LogSeverity::Debug)
		.buffer_size(1024)
		.build(log_path).unwrap();
	let mut log_arc = Arc::<Mutex<&mut Logger>>::from(Mutex::new(&mut log));

	//Attach a terminal listener.
	let term_listener = Arc::new(TerminalListenerBuilder::new()
		.build().unwrap());
	{
		let _ = log_arc.lock().unwrap().attach(term_listener);
	}
	//log_arc.unlock();

	let profiler = Profiler::create_global_instance(log_arc.clone()).unwrap();

	{
		log_arc.lock().unwrap().log_d("Beginning profiling test", "Game");
	}

	//Both GameBuilder::build()
	//and Game::run() are supposed to return Result,
	//so this should really store the result
	//and switch on the result value.
	let _ = GameBuilder::new()
		.build().unwrap()
		.run(&mut |_game|{
			for _ in 0..3 {
				profiler.begin_cpu_sample("test");
				thread::sleep(Duration::from_millis(1));
				profiler.end_cpu_sample();
			}
		}, &mut |_game|{});

	{
		log_arc.lock().unwrap().log_d("Profiling test complete", "Game");
	}
}