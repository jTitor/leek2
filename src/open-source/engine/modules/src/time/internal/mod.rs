pub mod windows_clock;
pub use self::windows_clock::WindowsClock;

pub mod posix_clock;
pub use self::posix_clock::PosixClock;

pub mod types;
pub use self::types::{DateTimeInternal, DateTimeInternalFactory};