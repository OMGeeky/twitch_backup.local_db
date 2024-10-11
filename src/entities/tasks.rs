use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, serde::Serialize)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub service_id: i32,
    pub description: Option<String>,
    pub progress: i32,
    pub max_progress: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::services::Entity",
        from = "Column::ServiceId",
        to = "super::services::Column::Id"
    )]
    Services,
}
impl Related<super::services::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Services.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
