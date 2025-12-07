use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Debug, Iden)]
enum GoogleCalendars {
    Table,
    RefreshTokenExpiresIn,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.alter_table(
            Table::alter()
                .table(GoogleCalendars::Table)
                .drop_column(GoogleCalendars::RefreshTokenExpiresIn)
                .to_owned(),
        )
        .await?;
        m.alter_table(
            Table::alter()
                .table(GoogleCalendars::Table)
                .add_column(
                    ColumnDef::new(GoogleCalendars::RefreshTokenExpiresIn)
                        .integer()
                        .null(),
                )
                .to_owned(),
        )
        .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.alter_table(
            Table::alter()
                .table(GoogleCalendars::Table)
                .drop_column(GoogleCalendars::RefreshTokenExpiresIn)
                .to_owned(),
        )
        .await?;
        m.alter_table(
            Table::alter()
                .table(GoogleCalendars::Table)
                .add_column(
                    ColumnDef::new(GoogleCalendars::RefreshTokenExpiresIn)
                        .integer()
                        .not_null(),
                )
                .to_owned(),
        )
        .await?;
        Ok(())
    }
}
