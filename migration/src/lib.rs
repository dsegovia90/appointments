#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]

pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20251014_222822_weekly_availabilities;
mod m20251015_012003_appointment_types;
mod m20251015_013448_appointments;
mod m20251030_084125_add_roles_to_user_table;
mod m20251030_091125_admin_settings;
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
            // inject-above (do not remove this comment)
        ]
    }
}
