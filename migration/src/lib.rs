#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]

pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20251014_222822_weekly_availabilities;
mod m20251015_012003_appointment_types;
mod m20251015_013448_appointments;
mod m20251030_084125_add_roles_to_user_table;
mod m20251030_091125_admin_settings;
mod m20251105_045209_google_calendars;
mod m20251105_051656_oauth_states;
mod m20251106_005544_add_google_calendar_json_to_admin_settings;
mod m20251106_105710_add_collision_and_events_to_google_calendars;
mod m20251111_073449_user_settings;
mod m20251119_034526_add_google_calendar_references_to_appointments;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20251014_222822_weekly_availabilities::Migration),
            Box::new(m20251015_012003_appointment_types::Migration),
            Box::new(m20251015_013448_appointments::Migration),
            Box::new(m20251030_084125_add_roles_to_user_table::Migration),
            Box::new(m20251030_091125_admin_settings::Migration),
            Box::new(m20251105_045209_google_calendars::Migration),
            Box::new(m20251105_051656_oauth_states::Migration),
            Box::new(m20251106_005544_add_google_calendar_json_to_admin_settings::Migration),
            Box::new(m20251106_105710_add_collision_and_events_to_google_calendars::Migration),
            Box::new(m20251111_073449_user_settings::Migration),
            Box::new(m20251119_034526_add_google_calendar_references_to_appointments::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}