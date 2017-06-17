/*!
	Entry point for testing leek2.
*/
extern crate leek2;
use leek2::GameBuilder;

fn main() {
	//Both GameBuilder::build()
	//and Game::run() are supposed to return Result,
	//so this should really store the result
	//and switch on the result value.
	GameBuilder::new()
		.build()?
		.run();
}