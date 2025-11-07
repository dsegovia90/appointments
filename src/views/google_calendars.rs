use std::collections::HashSet;

use google_calendar::types::CalendarListEntry;
use serde::{Deserialize, Serialize};

use crate::models::google_calendars;

#[derive(Debug, Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct CalendarEntry {
    access_role: String,
    background_color: String,
    foreground_color: String,
    id: String,
    primary: bool,
    selected: bool,
    summary: String,
    time_zone: String,
}

impl From<CalendarListEntry> for CalendarEntry {
    fn from(entry: CalendarListEntry) -> Self {
        Self {
            access_role: entry.access_role,
            background_color: entry.background_color,
            foreground_color: entry.foreground_color,
            id: entry.id,
            primary: entry.primary,
            selected: entry.selected,
            summary: entry.summary,
            time_zone: entry.time_zone,
        }
    }
}

#[derive(Debug, Deserialize, ts_rs::TS)]
pub enum CalendarSettingType {
    CollisionCheck,
    EventHandling,
}

#[derive(Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct CalendarSettingParams {
    pub calendar_id: String,
    pub setting_type: CalendarSettingType,
}

#[derive(Debug, Serialize, ts_rs::TS)]
#[ts(export)]
pub struct CalendarSettingsResponse {
    pub calendars_for_collision_check: HashSet<String>,
    pub calendars_for_event_handling: HashSet<String>,
}

impl From<google_calendars::Model> for CalendarSettingsResponse {
    fn from(model: google_calendars::Model) -> Self {
        Self {
            calendars_for_collision_check: model.calendars_for_collision_check.0,
            calendars_for_event_handling: model.calendars_for_event_handling.0,
        }
    }
}
