use crate::traits::GenericWindowComparison;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, ts_rs::TS)]
pub struct AvailabilityWindow {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl GenericWindowComparison<chrono::DateTime<Utc>> for AvailabilityWindow {
    fn start_time(&self) -> chrono::DateTime<Utc> {
        self.start
    }
    fn end_time(&self) -> chrono::DateTime<Utc> {
        self.end
    }
}

#[derive(Debug, Serialize, ts_rs::TS)]
#[ts(export)]
pub struct BookDay {
    pub day: DateTime<Utc>,
    pub availabilities: Vec<AvailabilityWindow>,
}

#[derive(Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct BookingParams {
    pub booker_name: String,
    pub booker_phone: String,
    pub booker_email: String,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}
