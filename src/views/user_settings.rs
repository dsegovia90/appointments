use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct UserSettingsProps {
    pub start_how_far_from_now_in_minutes: i32,
    pub end_how_far_from_now_in_minutes: i32,
}
