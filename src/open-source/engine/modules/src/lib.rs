//Crate root for the engine.
extern crate chrono;
extern crate gfx;
extern crate gfx_core;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate winapi;
extern crate kernel32;
extern crate num_traits;
extern crate rand;
extern crate remotery;
extern crate winit;

//Define our base level pub modules.

//Mathematics operations.
pub mod math;
//Import nearly_equal here since it's so commonly used.
pub use self::math::scalar::nearly_equal;

//Data and spatial organization structures.
//Generally agnostic about the data they store.
pub mod structures;

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