use crate::Clock;
use std::{
    fmt::{self, Debug},
    sync::RwLock,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Milliseconds since epoch at 2015-05-15T00:00:00 UTC.
///
/// This is the date of the Rust 1.0 announcement [here][rust-announcement].
///
/// [rust-announcement]: https://blog.rust-lang.org/2015/05/15/Rust-1.0.html
const DEFAULT_MILLIS_SINCE_EPOCH: u64 = 1431648000000;

/// Fake clock.
pub struct FakeClock {
    t: RwLock<SystemTime>,
}

impl FakeClock {
    /// Create a new [`FakeClock`] with the provided duration since [`UNIX_EPOCH`].
    ///
    /// [`FakeClock`]: struct.FakeClock.html
    /// [`UNIX_EPOCH`]: http://doc.rust-lang.org/std/time/constant.UNIX_EPOCH.html
    pub fn new(t: SystemTime) -> Self {
        Self { t: RwLock::new(t) }
    }

    /// Advance the clock by `duration`.
    pub fn advance(&self, duration: Duration) {
        let mut t = self.t.write().unwrap();
        *t += duration;
    }
}

impl Debug for FakeClock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FakeClock")
    }
}

/// Create a new [`FakeClock`] with the time `2015-05-15T00:00:00 UTC`.
impl Default for FakeClock {
    fn default() -> Self {
        Self::from(Duration::from_millis(DEFAULT_MILLIS_SINCE_EPOCH))
    }
}

impl Clone for FakeClock {
    fn clone(&self) -> Self {
        self.now().into()
    }
}

impl Clock for FakeClock {
    fn now(&self) -> SystemTime {
        *self.t.read().unwrap()
    }
}

/// Create a new [`FakeClock`] from the [`SystemTime`].
///
/// [`FakeClock`]: struct.FakeClock.html
/// [`SystemTime`]: http://doc.rust-lang.org/std/time/struct.SystemTime.html
impl From<SystemTime> for FakeClock {
    fn from(value: SystemTime) -> Self {
        Self { t: RwLock::new(value) }
    }
}

/// Create a new [`FakeClock`] from the duration duration since [`UNIX_EPOCH`].
///
/// [`FakeClock`]: struct.FakeClock.html
/// [`UNIX_EPOCH`]: http://doc.rust-lang.org/std/time/constant.UNIX_EPOCH.html
impl From<Duration> for FakeClock {
    fn from(value: Duration) -> Self {
        Self::from(UNIX_EPOCH + value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_system_time() {
        let now = SystemTime::now();
        let clock = FakeClock::from(now);
        assert_eq!(clock.now(), now);
    }

    #[test]
    fn from_duration() {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
        let clock = FakeClock::from(since_epoch);
        assert_eq!(clock.now(), now);
    }

    #[test]
    fn advance_secs() {
        let now = SystemTime::now();
        let clock = FakeClock::from(now);
        clock.advance(Duration::from_secs(1));
        assert_eq!(clock.now(), now + Duration::from_secs(1));
    }

    #[test]
    fn advance_nanos() {
        let now = SystemTime::now();
        let clock = FakeClock::from(now);
        clock.advance(Duration::from_nanos(1));
        assert_eq!(clock.now(), now + Duration::from_nanos(1));
    }
}
