use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Services::Table)
                    .col(
                        ColumnDef::new(Services::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Services::Name).string().not_null())
                    .col(ColumnDef::new(Services::LastUpdate).date_time().null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Tasks::Table)
                    .col(ColumnDef::new(Tasks::Id).integer().not_null())
                    .col(ColumnDef::new(Tasks::ServiceId).integer().not_null())
                    .col(ColumnDef::new(Tasks::Description).string().null())
                    .col(ColumnDef::new(Tasks::Progress).big_integer().default(0))
                    .col(ColumnDef::new(Tasks::MaxProgress).big_integer().default(0))
                    .primary_key(Index::create().col(Tasks::Id).col(Tasks::ServiceId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_task_service")
                            .from(Tasks::Table, Tasks::ServiceId)
                            .to(Services::Table, Services::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        _manager
            .drop_table(Table::drop().table(Services::Table).to_owned())
            .await?;
        _manager
            .drop_table(Table::drop().table(Tasks::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Services {
    Table,
    Id,
    Name,
    LastUpdate,
}

#[derive(Iden)]
pub enum Tasks {
    Table,
    Id,
    ServiceId,
    Description,
    Progress,
    MaxProgress,
}
