use anyhow::Context;
use local_db::entities::status::Status;
use local_db::entities::videos;
use local_db::print_db;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel};
use std::error::Error;
use tracing::{info, instrument};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().await.unwrap();

    Ok(())
}

async fn run() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter("sqlx=warn,sea_orm=warn")
        .init();
    info!("Hello, world!");
    let db = local_db::open_database(None).await?;
    local_db::reset_db(&db).await?;
    local_db::migrate_db(&db).await?;
    local_db::print_db(&db).await?;
    sample(&db).await?;
    info!("Bye!");
    Ok(())
}
#[instrument]
async fn sample(db: &DatabaseConnection) -> anyhow::Result<()> {
    let users = local_db::get_watched_users(db).await?;
    let user = users.first().context("Could not get any users...")?;
    info!("User: {:?}", user);
    let not_started_videos = local_db::get_videos_with_status(db, user, Status::NotStarted).await?;

    info!("Not started videos: {:?}", not_started_videos.len());
    let mut not_started_videos: Vec<videos::ActiveModel> = not_started_videos
        .into_iter()
        .map(|v| v.into_active_model())
        .collect();
    for video in not_started_videos.iter_mut() {
        info!("Video: {:?}", video);
        video.status = Set(Status::Downloading);
        video.clone().update(db).await?;
    }
    print_db(db).await?;

    for mut video in not_started_videos {
        info!("Video: {:?}", video);
        video.status = Set(Status::NotStarted);
        video.update(db).await?;
    }

    Ok(())
}