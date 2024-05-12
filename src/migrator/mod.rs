use sea_orm_migration::prelude::*;

mod m20230916_000001_create_users_table;
mod m20230916_000002_create_videos_table;
mod m20230916_000003_create_video_upload_table;
mod m20230916_000004_add_sample_data;
mod m20231015_000005_alter_videos_table_for_errors;
mod m20231029_000006_alter_video_uploads_table_for_youtube_id;
mod m20240509_000006_alter_users_table_for_timezone;

pub struct Migrator;
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230916_000001_create_users_table::Migration),
            Box::new(m20230916_000002_create_videos_table::Migration),
            Box::new(m20230916_000003_create_video_upload_table::Migration),
            Box::new(m20230916_000004_add_sample_data::Migration),
            Box::new(m20231015_000005_alter_videos_table_for_errors::Migration),
            Box::new(m20231029_000006_alter_video_uploads_table_for_youtube_id::Migration),
            Box::new(m20240509_000006_alter_users_table_for_timezone::Migration),
        ]
    }
}
