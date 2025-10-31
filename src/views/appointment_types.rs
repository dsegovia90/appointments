use serde::Deserialize;

#[derive(Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct CreateAppointmentTypeParams {
    pub duration_in_minutes: i32,
    pub display_name: String,
}

#[derive(Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct UpdateAppointmentTypeParams {
    pub duration_in_minutes: i32,
    pub display_name: String,
}
