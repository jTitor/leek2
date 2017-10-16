use graphics::{Device, Window};
use graphics::EventType;
use time::Clock;

/**
An instance of the game engine.
*/
#[derive(Debug)]
pub struct Game {
	graphics: Box<Device>,
	window: Box<Window>,
	clock: Box<Clock>
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