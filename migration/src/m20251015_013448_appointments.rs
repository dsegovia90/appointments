use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "appointments",
            &[
                ("id", ColType::PkAuto),
                ("booker_name", ColType::String),
                ("booker_phone", ColType::String),
                ("booker_timezone", ColType::String),
                ("booker_email", ColType::String),
                ("start_time", ColType::TimestampWithTimeZone),
                ("endtime", ColType::TimestampWithTimeZone),
                ("status", ColType::String),
            ],
            &[
                ("users", "user_id"),
                ("appointment_types", "appointment_type_id"),
            ],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "appointments").await
    }
}
