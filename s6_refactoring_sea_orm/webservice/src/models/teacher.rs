use sea_orm::{DeriveEntityModel, EnumIter};
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[derive(Serialize)]
#[sea_orm(table_name = "teacher")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    // 对应数据库的serial
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
