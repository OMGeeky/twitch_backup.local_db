#[cfg(feature = "sample-data")]
use crate::{
    migrator::m20230916_000001_create_users_table::Users,
    migrator::m20230916_000002_create_videos_table::Videos,
    migrator::m20230916_000003_create_video_upload_table::VideoUpload,
};
use sea_orm_migration::prelude::*;
use tracing::*;

#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    #[instrument(skip(_manager, self), name = "inserting_sample_data")]
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        #[cfg(feature = "sample-data")]
        {
            debug!("inserting sample data");
            for values in Values::get_users() {
                Self::add_user(_manager, values).await?;
                debug!("inserted user");
            }
            debug!("inserted user");
            for values in Values::get_videos() {
                Self::add_video(_manager, values).await?;
                debug!("inserted video");
            }
            for values in Values::get_video_uploads() {
                Migration::add_video_upload(_manager, values).await?;
                debug!("inserted video upload");
            }

            info!("inserted sample data");
        }
        Ok(())
    }
    #[instrument(skip(_manager, self), name = "delete_sample_data")]
    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        #[cfg(feature = "sample-data")]
        {
            info!("deleting sample data");
            for values in Values::get_video_uploads() {
                Migration::remove_video_upload(_manager, values).await?;
                debug!("removed video upload");
            }
            for values in Values::get_videos() {
                Migration::remove_video(_manager, values).await?;
                debug!("removed video");
            }
            for values in Values::get_users() {
                Migration::remove_user(_manager, values).await?;
                debug!("removed user");
            }
            info!("deleted sample data");
        }
        Ok(())
    }
}

#[cfg(feature = "sample-data")]
use s::Values;

#[cfg(feature = "sample-data")]
mod s {
    use super::*;
    const VIDEO_COLUMNS: [Videos; 15] = [
        Videos::Id,
        Videos::UserId,
        Videos::TwitchId,
        Videos::Name,
        Videos::TwitchPreviewImageUrl,
        Videos::TwitchDownloadUrl,
        Videos::Duration,
        Videos::CreatedAt,
        Videos::YoutubeId,
        Videos::YoutubePlaylistName,
        Videos::YoutubePreviewImageUrl,
        Videos::YoutubePlaylistId,
        Videos::YoutubePlaylistCreatedAt,
        Videos::PartCount,
        Videos::Status,
    ];
    const USER_COLUMNS: [Users; 10] = [
        Users::Id,
        Users::TwitchId,
        Users::TwitchName,
        Users::TwitchProfileImageUrl,
        Users::YoutubeId,
        Users::YoutubeName,
        Users::YoutubeProfileImageUrl,
        Users::YoutubeTargetDuration,
        Users::YoutubeMaxDuration,
        Users::Active,
    ];
    const VIDEO_UPLOAD_COLUMNS: [VideoUpload; 3] = [
        VideoUpload::VideoId,
        VideoUpload::Part,
        VideoUpload::UploadStatus,
    ];
    impl Migration {
        pub async fn add_video<'a>(
            manager: &SchemaManager<'a>,
            values: [SimpleExpr; 15],
        ) -> Result<(), DbErr> {
            let insert = Query::insert()
                .into_table(Videos::Table)
                .columns(VIDEO_COLUMNS)
                .values_panic(values)
                .to_owned();
            manager.exec_stmt(insert).await?;
            Ok(())
        }
        pub async fn remove_video<'a>(
            manager: &SchemaManager<'a>,
            values: [SimpleExpr; 15],
        ) -> Result<(), DbErr> {
            let columns = VIDEO_COLUMNS.map(Expr::col).to_vec();
            let mut statement = Query::delete();
            let insert = statement.from_table(Videos::Table);
            add_all_wheres(insert, values.to_vec(), columns);
            let insert = insert.to_owned();
            manager.exec_stmt(insert).await?;
            Ok(())
        }
        pub async fn remove_user<'a>(
            manager: &SchemaManager<'a>,
            values: [SimpleExpr; 10],
        ) -> Result<(), DbErr> {
            let columns = USER_COLUMNS.map(Expr::col).to_vec();
            let mut statement = Query::delete();
            let insert = statement.from_table(Users::Table);
            add_all_wheres(insert, values.to_vec(), columns);
            let insert = insert.to_owned();
            manager.exec_stmt(insert).await?;
            Ok(())
        }
        pub async fn remove_video_upload<'a>(
            manager: &SchemaManager<'a>,
            values: [SimpleExpr; 3],
        ) -> Result<(), DbErr> {
            let columns = VIDEO_UPLOAD_COLUMNS.map(Expr::col).to_vec();
            let mut statement = Query::delete();
            let insert = statement.from_table(VideoUpload::Table);
            add_all_wheres(insert, values.to_vec(), columns);
            let insert = insert.to_owned();
            manager.exec_stmt(insert).await?;
            Ok(())
        }

        pub async fn add_user<'a>(
            manager: &SchemaManager<'a>,
            parameters: [SimpleExpr; 10],
        ) -> Result<(), DbErr> {
            let insert = Query::insert()
                .into_table(Users::Table)
                .columns(USER_COLUMNS)
                .values_panic(parameters)
                .to_owned();
            manager.exec_stmt(insert).await?;
            Ok(())
        }

        pub async fn add_video_upload<'a>(
            manager: &SchemaManager<'a>,
            parameters: [SimpleExpr; 3],
        ) -> Result<(), DbErr> {
            let insert = Query::insert()
                .into_table(VideoUpload::Table)
                .columns(VIDEO_UPLOAD_COLUMNS)
                .values_panic(parameters)
                .to_owned();
            manager.exec_stmt(insert).await?;
            Ok(())
        }
    }
    pub fn add_all_wheres(
        query: &mut DeleteStatement,
        values: Vec<SimpleExpr>,
        columns: Vec<Expr>,
    ) {
        for (val, col) in values.into_iter().zip(columns.into_iter()) {
            add_where(query, val, col);
        }
    }
    pub fn add_where(query: &mut DeleteStatement, value: SimpleExpr, column: Expr) {
        query.and_where(column.eq(value));
    }
    pub struct Values;
    impl Values {
        pub fn get_users() -> Vec<[SimpleExpr; 10]> {
            vec![[
                3.into(),
                "123".into(),
                "test".into(),
                "test".into(),
                "123".into(),
                "test".into(),
                "test".into(),
                1.into(),
                1.into(),
                true.into(),
            ]]
        }
        pub fn get_videos() -> Vec<[SimpleExpr; 15]> {
            vec![
                [
                    1.into(),
                    3.into(),
                    "video2".into(),
                    "test".into(),
                    "test".into(),
                    "test".into(),
                    1.into(),
                    "test".into(),
                    "123".into(),
                    "test".into(),
                    "test".into(),
                    "test".into(),
                    "test".into(),
                    1.into(),
                    0.into(),
                ],
                [
                    2.into(),
                    3.into(),
                    "123".into(),
                    "test".into(),
                    "test".into(),
                    "test".into(),
                    1.into(),
                    "test".into(),
                    "123".into(),
                    "test".into(),
                    "test".into(),
                    "test".into(),
                    "test".into(),
                    1.into(),
                    0.into(),
                ],
                [
                    3.into(),
                    3.into(),
                    "asdf123".into(),
                    "test".into(),
                    "test".into(),
                    "test".into(),
                    1.into(),
                    "test".into(),
                    "123".into(),
                    "test".into(),
                    "test".into(),
                    "test".into(),
                    "test".into(),
                    1.into(),
                    100.into(),
                ],
            ]
        }
        pub fn get_video_uploads() -> Vec<[SimpleExpr; 3]> {
            vec![
                [1.into(), 1.into(), 0.into()],
                [1.into(), 2.into(), 0.into()],
                [2.into(), 1.into(), 0.into()],
                [2.into(), 2.into(), 0.into()],
                [2.into(), 3.into(), 0.into()],
            ]
        }
    }
}
