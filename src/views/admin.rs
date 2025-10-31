use crate::{
    models::weekly_availabilities::{self},
    traits::GenericWindowComparison,
};
use serde::{Deserialize, Serialize};

/// Normalizes availability to 0 minutes min and 1440 minutes max
#[derive(Debug, Deserialize, Serialize, ts_rs::TS)]
pub struct NormalizedAvailabilityDay {
    pub from: i32,
    pub to: i32,
}

#[derive(Debug, Serialize, ts_rs::TS)]
pub struct AvailabilityDay {
    pub normalized: NormalizedAvailabilityDay,
    pub model: weekly_availabilities::Model,
}

#[derive(Debug, Serialize, ts_rs::TS)]
#[ts(export)]
pub struct WeeklyAvailabilityByWeekday {
    pub monday: Vec<AvailabilityDay>,
    pub tuesday: Vec<AvailabilityDay>,
    pub wednesday: Vec<AvailabilityDay>,
    pub thursday: Vec<AvailabilityDay>,
    pub friday: Vec<AvailabilityDay>,
    pub saturday: Vec<AvailabilityDay>,
    pub sunday: Vec<AvailabilityDay>,
}

impl WeeklyAvailabilityByWeekday {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            monday: vec![],
            tuesday: vec![],
            wednesday: vec![],
            thursday: vec![],
            friday: vec![],
            saturday: vec![],
            sunday: vec![],
        }
    }

    pub fn add_availability_by_day_index(&mut self, item: weekly_availabilities::Model) {
        let day_index = item.from / (60 * 24);
        let normalized = NormalizedAvailabilityDay {
            from: item.from - day_index * (60 * 24),
            to: item.to - day_index * (60 * 24),
        };

        match day_index {
            0 => self.monday.push(AvailabilityDay {
                model: item,
                normalized,
            }),
            1 => self.tuesday.push(AvailabilityDay {
                model: item,
                normalized,
            }),
            2 => self.wednesday.push(AvailabilityDay {
                model: item,
                normalized,
            }),
            3 => self.thursday.push(AvailabilityDay {
                model: item,
                normalized,
            }),
            4 => self.friday.push(AvailabilityDay {
                model: item,
                normalized,
            }),
            5 => self.saturday.push(AvailabilityDay {
                model: item,
                normalized,
            }),
            6 => self.sunday.push(AvailabilityDay {
                model: item,
                normalized,
            }),
            _ => {}
        }
    }
}

impl Default for WeeklyAvailabilityByWeekday {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize, Debug, ts_rs::TS)]
#[ts(export)]
pub struct WeeklyAvailabilitiesCreateUpdateParams {
    pub normalized: NormalizedAvailabilityDay,
    /// Zero indexed weekday (0 = Monday, 1 = Tuesday, etc.)
    pub weekday: i32,
}

impl GenericWindowComparison<i32> for WeeklyAvailabilitiesCreateUpdateParams {
    fn start_time(&self) -> i32 {
        self.normalized.from + self.weekday * (60 * 24)
    }
    fn end_time(&self) -> i32 {
        self.normalized.to + self.weekday * (60 * 24)
    }
}
