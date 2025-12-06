use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Appointments {
    Table,
    GoogleCalendarEvents,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.alter_table(
            Table::alter()
                .table(Appointments::Table)
                .add_column_if_not_exists(
                    ColumnDef::new(Appointments::GoogleCalendarEvents)
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
                .table(Appointments::Table)
                .drop_column(Appointments::GoogleCalendarEvents)
                .to_owned(),
        )
        .await?;
        Ok(())
    }
}
