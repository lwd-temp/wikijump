//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use super::sea_orm_active_enums::MessageRecipientType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "message_recipient")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub record_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub recipient_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub recipient_type: MessageRecipientType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::message_record::Entity",
        from = "Column::RecordId",
        to = "super::message_record::Column::ExternalId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    MessageRecord,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::RecipientId",
        to = "super::user::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::message_record::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MessageRecord.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
