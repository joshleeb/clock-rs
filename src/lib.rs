use std::time::SystemTime;

/// Simple clock.
pub trait Clock {
    /// Returns the time corresponding to "now".
    fn now(&self) -> SystemTime;
}
