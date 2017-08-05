/*!
	Creates an instance of the game engine.
*/
mod errors;

use super::graphics::{Device, DeviceBuilder};
use super::graphics::{Window, WindowBuilder};
use super::graphics::EventType;
use self::errors::GameError;

/**
An instance of the game engine.
*/
#[derive(Debug)]
pub struct Game {
	graphics: Box<Device>,
	window: Box<Window>
	//unimplemented!()
}

impl Game {
	/**
	Initializes the game and enters the game loop.
	*/
	pub fn run(&mut self) -> Result<(), GameError> {
		//Enter the game loop here.
		let mut running = true;
		while running {
			//Update our input.
			let mut escPressed = false;
			//We do this by getting the window's events...
			for event in self.window.poll_events() {
				match *event {
					EventType::Closed => escPressed = true,
					_ => {}
				}
			}

			//Do update and render here.

			//For now, quit on ESC.
			running = !escPressed;
		}

		Ok(())
	}
}

impl Drop for Game {
	fn drop(&mut self) {
		//Disconnect devices here.
		//Failures aren't fatal,
		//but may be reported.

		//Close the window.
		self.window.hide();
		//Disconnect the window.
		//Disconnect the graphics device.
		unimplemented!()
	}
}

/**
Generates instances of the game engine.
*/
#[derive(Debug)]
pub struct GameBuilder {
	//unimplemented!()
}

impl GameBuilder {
	pub fn new() -> GameBuilder {
		GameBuilder {
		}
	}

	pub fn build(&self) -> Result<Game, GameError> {
		//Initialize devices here.
		//Graphics device...
		//Given our platform, figure out
		//the best backend for the device.
		
		//TODO: Consider making Game<T> templated
		//on backend type to avoid polymorphing
		//on graphics calls?
		let graphics = DeviceBuilder::new()
			.build_automatic_backend()?;
		//Then the window.
		let window = WindowBuilder::new()
			.build(&*graphics)?;
		//Open the window here.
		window.open();
		//Devices ready, assign them to the game.
		unimplemented!();
		Ok(Game {
			graphics: graphics,
			window: window
		})
	}
}