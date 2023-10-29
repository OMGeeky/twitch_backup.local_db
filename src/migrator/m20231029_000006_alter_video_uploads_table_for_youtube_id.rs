use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(VideoUpload::Table)
                    .add_column(
                        ColumnDef::new(VideoUpload::YoutubeVideoId)
                            .string()
                            .null()
                            .default(None::<String>),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(VideoUpload::Table)
                    .drop_column(VideoUpload::YoutubeVideoId)
                    .to_owned(),
            )
            .await
    }
}
#[derive(Iden)]
pub enum VideoUpload {
    Table,
    YoutubeVideoId,
}
