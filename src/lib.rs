use crate::entities::video_upload::UploadStatus;
use crate::{
    entities::prelude::{Users, VideoUpload, Videos},
    entities::status::Status,
    entities::*,
    migrator::Migrator,
};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, DbErr, EntityTrait, ModelTrait,
    QueryFilter,
};
use sea_orm_migration::MigratorTrait;
use tracing::{info, instrument};

pub mod entities;
mod migrator;

pub const DATABASE_URL: &str = "sqlite:./data.db?mode=rwc";
/// Open a database connection. If no url is provided, the default [DATABASE_URL] is used.
#[instrument]
pub async fn open_database(db_url: Option<&str>) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(db_url.unwrap_or(DATABASE_URL)).await?;
    Ok(db)
}
/// Removes all applied migrations. This will remove most data from the database.
pub async fn reset_db(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::reset(db).await?;
    Ok(())
}
/// Applies all migrations that are not already applied.
pub async fn migrate_db(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::refresh(db).await?;
    Ok(())
}

/// Prints all entries in the database. This is for debugging.
#[instrument]
pub async fn print_db(db: &DatabaseConnection) -> Result<(), DbErr> {
    info!("Printing DB");
    let users = Users::find().all(db).await?;

    info!("Relations:");

    for u in users.iter() {
        dbg!(&u);
        for v in u.find_related(Videos).all(db).await.iter() {
            for video in v.iter() {
                dbg!(&video);
                video
                    .find_related(VideoUpload)
                    .all(db)
                    .await
                    .iter()
                    .for_each(|vu| {
                        dbg!(vu);
                    });
            }
        }
    }

    info!("Done!");
    Ok(())
}

pub async fn get_watched_users(db: &DatabaseConnection) -> Result<Vec<users::Model>, DbErr> {
    let users = Users::find()
        .filter(users::Column::Active.eq(true))
        .all(db)
        .await?;
    Ok(users)
}

pub async fn get_videos_with_status<'a, U: Into<&'a users::Model>>(
    db: &DatabaseConnection,
    user: U,
    status: Status,
) -> Result<Vec<videos::Model>, DbErr> {
    let videos = user
        .into()
        .find_related(Videos)
        .filter(videos::Column::Status.eq(status))
        .all(db)
        .await?;
    Ok(videos)
}

pub async fn get_video_uploads<'a, V: Into<&'a videos::Model>>(
    db: &DatabaseConnection,
    video: V,
) -> Result<Vec<video_upload::Model>, DbErr> {
    let uploads = video.into().find_related(VideoUpload).all(db).await?;
    Ok(uploads)
}
pub async fn create_video_upload<'a, V: Into<&'a videos::Model>>(
    db: &DatabaseConnection,
    video: V,
    part: i32,
) -> Result<video_upload::Model, DbErr> {
    let video = video.into();
    let upload = video_upload::ActiveModel {
        video_id: Set(video.id),
        part: Set(part),
        upload_status: Set(UploadStatus::Pending),
    };
    let upload = upload.insert(db).await?;
    Ok(upload)
}

//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
