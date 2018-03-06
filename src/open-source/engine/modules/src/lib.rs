//Crate root for the engine.

//This would require one of the feature modes to be set;
//since we default to GL anyway this isn't enabled yet.
// #![cfg_attr(
// 	not(any(feature = "vulkan", feature = "dx12", feature = "metal", feature = "gl")),
// 	allow(dead_code, unused_extern_crates, unused_imports)
// )]

extern crate chrono;
extern crate failure;
//extern crate gfx;
extern crate gfx_hal;
//#[cfg(feature = "dx12")]
//extern crate gfx_backend_dx12;
//#[cfg(feature = "vulkan")]
//extern crate gfx_backend_vulkan;
//#[cfg(feature = "metal")]
//extern crate gfx_backend_metal;
//#[cfg(feature = "gl")]
extern crate gfx_backend_gl;
extern crate glutin;
extern crate image;
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