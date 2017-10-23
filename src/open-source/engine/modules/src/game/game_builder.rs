use game::Game;
use game::GameError;
use graphics::{GraphicsFactory, GraphicsPayload};
use graphics::{Device, Window};
use time::ClockFactory;

/**
Generates instances of the game engine.
*/
#[derive(Debug)]
pub struct GameBuilder {
	//unimplemented!()
}

impl GameBuilder {
	pub fn new() -> GameBuilder {
		GameBuilder {}
	}

	pub fn build(&self) -> Result<Game, GameError> {
		//Initialize devices here.		
		//TODO: Consider making Game<T> templated
		//on backend type to avoid polymorphing
		//on graphics calls?
		let graphics_payload = GraphicsFactory::new()
			.build()?;
		let clock = ClockFactory::new()
			.build()?;
		//Open the window here.
		graphics_payload.window.open();
		//Devices ready, assign them to the game.
		unimplemented!();
		Ok(Game {
			graphics: graphics_payload.device,
			window: graphics_payload.window,
			clock: Box::new(clock)
		})
	}
}