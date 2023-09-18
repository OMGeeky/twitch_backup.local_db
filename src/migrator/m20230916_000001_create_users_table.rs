use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::TwitchId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::TwitchName).string().not_null())
                    .col(ColumnDef::new(Users::TwitchProfileImageUrl).string().null())
                    .col(ColumnDef::new(Users::YoutubeId).string().not_null())
                    .col(ColumnDef::new(Users::YoutubeName).string().not_null())
                    .col(
                        ColumnDef::new(Users::YoutubeProfileImageUrl)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Users::YoutubeTargetDuration)
                            .integer()
                            .not_null()
                            .comment(
                                "The target duration of a video. If \
                                    the duration of a part would be \
                                    longer than the max duration the current \
                                    part will be cut to this duration and the \
                                    rest will be uploaded as a new part \
                                    (splitting it again if needed).",
                            ),
                    )
                    .col(
                        ColumnDef::new(Users::YoutubeMaxDuration)
                            .integer()
                            .not_null()
                            .comment(
                                "The maximum duration \
                                    an upload should be. If the \
                                    duration is longer than this, it will \
                                    be split into multiple parts.",
                            ),
                    )
                    .col(ColumnDef::new(Users::Active).boolean().not_null())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        _manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}
#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    TwitchId,
    TwitchName,
    TwitchProfileImageUrl,
    YoutubeId,
    YoutubeName,
    YoutubeProfileImageUrl,
    YoutubeTargetDuration,
    YoutubeMaxDuration,
    Active,
}
