//Crate root for the engine.
#[macro_use]
extern crate chrono;
extern crate gfx;
extern crate gfx_core;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;

//Define our base level pub modules.

//Mathematics operations.
pub mod math;

//Graphics functions and interfaces.
pub mod graphics;

//Audio functions and interfaces.
pub mod audio;

//Physics functions and interfaces.
pub mod physics;

//Input management.
pub mod input;

//Memory allocator and memory
//allocation functions.
pub mod memory;

//Asset management.
pub mod asset;

//Logging operations.
pub mod logging;

//Platform-dependent subsystems.
pub mod platform;

pub mod time;

//Instantiator for the engine.
pub mod game;

pub use game::{Game, GameBuilder};