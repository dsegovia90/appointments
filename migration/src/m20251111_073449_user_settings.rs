use loco_rs::schema::table_auto_tz;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum UserSettings {
    Table,
    Id,
    UserId,
    StartHowFarFromNowInMinutes,
    EndHowFarFromNowInMinutes,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            table_auto_tz(UserSettings::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(UserSettings::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(UserSettings::UserId)
                        .integer()
                        .not_null()
                        .unique_key(),
                )
                .col(
                    ColumnDef::new(UserSettings::StartHowFarFromNowInMinutes)
                        .integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(UserSettings::EndHowFarFromNowInMinutes)
                        .integer()
                        .not_null(),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-user-settings-user_id")
                        .from(UserSettings::Table, UserSettings::UserId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(UserSettings::Table).to_owned())
            .await?;
        Ok(())
    }
}
