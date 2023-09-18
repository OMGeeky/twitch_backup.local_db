use crate::migrator::m20230916_000001_create_users_table::Users;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Videos::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Videos::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Videos::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_id")
                            .to(Users::Table, Users::Id)
                            .from(Videos::Table, Videos::UserId),
                    )
                    .col(
                        ColumnDef::new(Videos::TwitchId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Videos::Name).string().not_null())
                    .col(
                        ColumnDef::new(Videos::TwitchPreviewImageUrl)
                            .string()
                            .null(),
                    )
                    .col(ColumnDef::new(Videos::TwitchDownloadUrl).string().null())
                    .col(ColumnDef::new(Videos::Duration).integer().not_null())
                    .col(ColumnDef::new(Videos::CreatedAt).date_time().not_null())
                    //Youtube stuff
                    .col(ColumnDef::new(Videos::YoutubeId).string().null())
                    .col(
                        ColumnDef::new(Videos::YoutubePlaylistName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Videos::YoutubePreviewImageUrl)
                            .string()
                            .null(),
                    )
                    .col(ColumnDef::new(Videos::YoutubePlaylistId).string().null())
                    .col(
                        ColumnDef::new(Videos::YoutubePlaylistCreatedAt)
                            .date_time()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Videos::PartCount)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(Videos::Status)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        _manager
            .drop_table(Table::drop().table(Videos::Table).to_owned())
            .await
    }
}
#[derive(Iden)]
pub enum Videos {
    Table,
    Id,
    UserId,
    TwitchId,
    Name,
    Duration,
    CreatedAt,
    TwitchDownloadUrl,
    TwitchPreviewImageUrl,
    YoutubeId,
    YoutubePlaylistName,
    YoutubePreviewImageUrl,
    YoutubePlaylistId,
    YoutubePlaylistCreatedAt,
    PartCount,
    Status,
}
