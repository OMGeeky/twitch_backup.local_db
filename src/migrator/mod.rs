use sea_orm_migration::prelude::*;

mod m20230916_000001_create_users_table;
mod m20230916_000002_create_videos_table;
mod m20230916_000003_create_video_upload_table;
mod m20230916_000004_add_sample_data;

pub struct Migrator;
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230916_000001_create_users_table::Migration),
            Box::new(m20230916_000002_create_videos_table::Migration),
            Box::new(m20230916_000003_create_video_upload_table::Migration),
            // Box::new(m20230916_000004_add_sample_data::Migration),
        ]
    }
}
