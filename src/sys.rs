use crate::Clock;
use std::time::SystemTime;

/// System clock.
///
/// Uses [`SystemTime`] internally.
///
/// [`SystemTime`]: http://doc.rust-lang.org/std/time/struct.SystemTime.html
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}
