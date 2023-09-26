//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub twitch_id: String,
    pub twitch_name: String,
    pub twitch_profile_image_url: Option<String>,
    pub youtube_id: String,
    pub youtube_name: String,
    pub youtube_profile_image_url: Option<String>,
    /// The target duration of a video. If
    /// the duration of a part would be
    /// longer than the max duration the current
    /// part will be cut to this duration and the
    /// rest will be uploaded as a new part
    /// (splitting it again if needed).
    pub youtube_target_duration: i32,
    /// The maximum duration
    /// an upload should be. If the
    /// duration is longer than this, it will
    /// be split into multiple parts.
    pub youtube_max_duration: i32,
    pub active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::videos::Entity")]
    Videos,
}
impl Related<super::videos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Videos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}