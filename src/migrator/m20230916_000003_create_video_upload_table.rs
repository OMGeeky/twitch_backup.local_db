use crate::migrator::m20230916_000002_create_videos_table::Videos;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VideoUpload::Table)
                    .col(ColumnDef::new(VideoUpload::VideoId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_video_id")
                            .to(Videos::Table, Videos::Id)
                            .from(VideoUpload::Table, VideoUpload::VideoId),
                    )
                    .col(ColumnDef::new(VideoUpload::Part).integer().not_null())
                    .col(
                        ColumnDef::new(VideoUpload::UploadStatus)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(VideoUpload::VideoId)
                            .col(VideoUpload::Part),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        _manager
            .drop_table(Table::drop().table(VideoUpload::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum VideoUpload {
    Table,
    VideoId,
    Part,
    UploadStatus,
}
