#[cfg(feature = "mock-time")]
use std::sync::Mutex;

use chrono::{self, DateTime, Utc};

#[cfg(not(feature = "mock-time"))]
pub fn utc_now() -> DateTime<Utc> {
    chrono::Utc::now()
}

#[cfg(feature = "mock-time")]
pub static CHRONO_NOW: std::sync::LazyLock<Mutex<DateTime<Utc>>> =
    std::sync::LazyLock::new(|| Mutex::new(chrono::Utc::now()));

#[cfg(feature = "mock-time")]
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn utc_now() -> DateTime<Utc> {
    *CHRONO_NOW.lock().unwrap()
}
