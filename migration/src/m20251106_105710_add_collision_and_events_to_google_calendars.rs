use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum GoogleCalendars {
    Table,
    CalendarsForCollisionCheck,
    CalendarsForEventHandling,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.alter_table(
            Table::alter()
                .table(GoogleCalendars::Table)
                .add_column(
                    ColumnDef::new(GoogleCalendars::CalendarsForCollisionCheck)
                        .json_binary()
                        .not_null()
                        .default(Expr::value("[]")),
                )
                .to_owned(),
        )
        .await?;

        m.alter_table(
            Table::alter()
                .table(GoogleCalendars::Table)
                .add_column(
                    ColumnDef::new(GoogleCalendars::CalendarsForEventHandling)
                        .json_binary()
                        .not_null()
                        .default(Expr::value("[]")),
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
                .drop_column(GoogleCalendars::CalendarsForCollisionCheck)
                .to_owned(),
        )
        .await?;
        m.alter_table(
            Table::alter()
                .table(GoogleCalendars::Table)
                .drop_column(GoogleCalendars::CalendarsForEventHandling)
                .to_owned(),
        )
        .await?;
        Ok(())
    }
}
