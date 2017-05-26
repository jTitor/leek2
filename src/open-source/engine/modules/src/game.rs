/*!
	Creates an instance of the game engine.
*/

/**
An instance of the game engine.
*/
pub struct Game {
	unimplemented!()
}

impl Game {
	/**
	Initializes the game and enters the game loop.
	*/
	pub fn run(&mut self) {
		//Enter the game loop here.
		unimplemented!()
	}
}

impl Drop for Game {
	fn drop(&mut self) {
		//Disconnect devices here.
		//Failures aren't fatal,
		//but may be reported.

		//Close the window.
		//Disconnect the window.
		//Disconnect the graphics device.
		unimplemented!()
	}
}

/**
Generates instances of the game engine.
*/
pub struct GameBuilder {
	unimplemented!()
}

impl GameBuilder {
	pub fn new() -> GameBuilder {
		GameBuilder {
		}
	}

	pub fn build(&self) -> Result<Game, Error> {
		//Initialize devices here.
		//Graphics device...
		//Then the window.
		//Open the window here.
		unimplemented!()
	}
}