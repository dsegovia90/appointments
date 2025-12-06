use crate::models::{_entities::appointments::Status, appointments};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

const fn default_limit() -> u64 {
    10
}

#[derive(Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct AppointmentsQueryParams {
    #[serde(default)]
    pub page: u64,
    #[serde(default = "default_limit")]
    pub limit: u64,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub status: Option<Status>,
    pub appointment_type: Option<i32>,
}

#[derive(Debug, Serialize, ts_rs::TS)]
#[ts(export)]
pub struct AppointmentsResponse {
    pub appointments: Vec<appointments::Model>,
    pub count: u64,
}
