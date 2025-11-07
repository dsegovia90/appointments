use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "google_calendars",
            &[
                ("id", ColType::PkAuto),
                ("access_token", ColType::String),
                ("expires_in", ColType::Integer),
                ("refresh_token", ColType::String),
                ("refresh_token_expires_in", ColType::Integer),
                ("scope", ColType::String),
                ("token_type", ColType::String),
            ],
            &[("users", "user_id")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "google_calendars").await
    }
}
