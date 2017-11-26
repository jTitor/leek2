extern crate log;
extern crate leek2;

pub mod logging;
pub mod time;
pub mod graphics;
pub mod math;
//Import nearly_equal here since it's so commonly used.
pub use self::math::scalar::nearly_equal;