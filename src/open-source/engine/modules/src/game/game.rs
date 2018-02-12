use game::GameError;
use graphics::{Device, Window};
use graphics::EventType;
use time::Clock;

/**
An instance of the game engine.
*/
#[derive(Debug)]
pub struct Game {
	pub graphics: Box<Device>,
	pub window: Box<Window>,
	pub clock: Box<Clock>
	//unimplemented!()
}

impl Game {
	/**
	Initializes the game and enters the game loop.
	*/
	pub fn run(&mut self) -> Result<(), GameError> {
		self.window.open();
		
		//Enter the game loop here.
		let mut running = true;
		while running {
			//Update our input.
			let mut esc_pressed = false;
			//We do this by getting the window's events...
			self.window.poll_events(&mut |event| {
				match event {
					EventType::Closed => { esc_pressed = true; },
					EventType::ReceivedCharacter(char_received) => {
						let _unused = char_received;
						//TODO: match on Esc key specifically
						esc_pressed = true;
					},
					_ => {}
				}
			});

			//Do update and render here.
			self.clock.update();

			//For now, quit on ESC.
			running = !esc_pressed;
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
		//unimplemented!()
	}
}