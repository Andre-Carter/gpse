use std::time::{Instant, SystemTime};

pub fn system_time_now() -> SystemTime {
    SystemTime::now()
}

pub fn monotonic_time_now() -> Instant {
    Instant::now()
}
