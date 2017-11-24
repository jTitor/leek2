/*!
 Module for time measurements.
 */

pub mod types;
pub use self::types::{TimeDuration, TimeRange, TimeStamp, DateTime, TimeElement, ClockType};

pub mod clock;
pub use self::clock::Clock;

mod internal;

pub mod clock_factory;
pub use self::clock_factory::ClockFactory;