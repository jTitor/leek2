/*!
 Module for time measurements.
 */

pub mod types;
pub use self::types::{TimeDuration, TimeRange, TimeStamp, DateTime};

pub mod clock;
pub use self::clock::{Clock, ClockFactory};

mod internal;