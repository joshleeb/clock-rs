pub use fake::FakeClock;
pub use sys::SystemClock;

use std::time::SystemTime;

mod fake;
mod sys;

/// Simple clock.
pub trait Clock {
    /// Returns the time corresponding to "now".
    fn now(&self) -> SystemTime;
}
