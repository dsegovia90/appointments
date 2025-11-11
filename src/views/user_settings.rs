use serde::Deserialize;

#[derive(Debug, Deserialize, ts_rs::TS)]
pub struct DaysHoursMinutes {
    pub days: i32,
    pub hours: i32,
    pub minutes: i32,
}

impl DaysHoursMinutes {
    const HOURS_IN_DAY: i32 = 24;
    const MINUTES_IN_HOUR: i32 = 60;

    #[must_use]
    pub const fn as_minutes(&self) -> i32 {
        self.days * Self::HOURS_IN_DAY * Self::MINUTES_IN_HOUR
            + self.hours * Self::MINUTES_IN_HOUR
            + self.minutes
    }
}

#[derive(Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct UserSettingsProps {
    pub start_how_far_from_now: DaysHoursMinutes,
    pub end_how_far_from_now: DaysHoursMinutes,
}
