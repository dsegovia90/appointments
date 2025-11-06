use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum AdminSettings {
    Table,
    GoogleCloudApiKey,
    GoogleCalendarSettings,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.alter_table(
            Table::alter()
                .table(AdminSettings::Table)
                .drop_column(AdminSettings::GoogleCloudApiKey)
                .to_owned(),
        )
        .await?;

        let column = ColumnDef::new(AdminSettings::GoogleCalendarSettings)
            .json_binary()
            .to_owned();

        m.alter_table(
            Table::alter()
                .table(AdminSettings::Table)
                .add_column(column)
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.alter_table(
            Table::alter()
                .table(AdminSettings::Table)
                .add_column(
                    ColumnDef::new(AdminSettings::GoogleCloudApiKey)
                        .string()
                        .null()
                        .to_owned(),
                )
                .to_owned(),
        )
        .await?;

        m.alter_table(
            Table::alter()
                .table(AdminSettings::Table)
                .drop_column(AdminSettings::GoogleCalendarSettings)
                .to_owned(),
        )
        .await?;

        Ok(())
    }
}
