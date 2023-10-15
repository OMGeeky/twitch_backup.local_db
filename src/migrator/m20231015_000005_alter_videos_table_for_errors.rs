use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Videos::Table)
                    .add_column(
                        ColumnDef::new(Videos::FailCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Videos::Table)
                    .add_column(ColumnDef::new(Videos::FailReason).string().null())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Videos::Table)
                    .drop_column(Videos::FailCount)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Videos::Table)
                    .drop_column(Videos::FailReason)
                    .to_owned(),
            )
            .await
    }
}
#[derive(Iden)]
pub enum Videos {
    Table,
    FailCount,
    FailReason,
}
