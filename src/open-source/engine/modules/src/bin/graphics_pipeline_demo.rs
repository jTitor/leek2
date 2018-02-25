/*!
 * Demonstrates the internal graphics structs.
 * 
 * The Device abstraction isn't fully implemented, so
 * this is used as a compromise to test out graphics functionality in the meantime.
 */
xtern crate leek2;
use leek2::GameBuilder;
use leek2::logging::profiling::{Profiler, ProfileScope};
use leek2::logging::{Logger, LoggerBuilder, LogSeverity, TerminalListenerBuilder};
use leek2::time::{Clock, ClockFactory};
use std::time::Duration;
use std::thread;
use std::sync::{Arc, RwLock};

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
	let mut log_arc = Arc::<RwLock<&mut Logger>>::from(RwLock::new(&mut log));

	//Attach a terminal listener.
	let term_listener = Arc::new(TerminalListenerBuilder::new()
		.build().unwrap());
	{
		let _ = log_arc.lock().unwrap().attach(term_listener);
	}
	//log_arc.unlock();

	let profiler = Profiler::create_global_instance(log_arc.clone()).unwrap();

	{
		log_arc.lock().unwrap().log_d("Beginning graphics test", "Game");
	}

	unimplemented!();
	//TODO_rust: Setup the graphics structs here.

	//Both GameBuilder::build()
	//and Game::run() are supposed to return Result,
	//so this should really store the result
	//and switch on the result value.
	let _ = GameBuilder::new()
		.build().unwrap()
		.run(&mut |_game|{
			unimplemented!();

			//TODO_rust: perform any updating here
		}, &mut |_game|{
			unimplemented!();

			//TODO_rust: submit the draw calls here
		});

	//TODO_rust:
	//perform any graphics shutdown here

	{
		log_arc.lock().unwrap().log_d("Graphics test complete", "Game");
	}
}