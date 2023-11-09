//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "alias_type")]
pub enum AliasType {
    #[sea_orm(string_value = "site")]
    Site,
    #[sea_orm(string_value = "user")]
    User,
}
#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "file_revision_type")]
pub enum FileRevisionType {
    #[sea_orm(string_value = "create")]
    Create,
    #[sea_orm(string_value = "delete")]
    Delete,
    #[sea_orm(string_value = "undelete")]
    Undelete,
    #[sea_orm(string_value = "update")]
    Update,
}
#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "interaction_object_type"
)]
pub enum InteractionObjectType {
    #[sea_orm(string_value = "file")]
    File,
    #[sea_orm(string_value = "page")]
    Page,
    #[sea_orm(string_value = "site")]
    Site,
    #[sea_orm(string_value = "user")]
    User,
}
#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "message_recipient_type"
)]
pub enum MessageRecipientType {
    #[sea_orm(string_value = "bcc")]
    Bcc,
    #[sea_orm(string_value = "cc")]
    Cc,
    #[sea_orm(string_value = "regular")]
    Regular,
}
#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "page_revision_type")]
#[serde(rename_all = "kebab-case")]
pub enum PageRevisionType {
    #[sea_orm(string_value = "create")]
    Create,
    #[sea_orm(string_value = "delete")]
    Delete,
    #[sea_orm(string_value = "move")]
    Move,
    #[sea_orm(string_value = "regular")]
    Regular,
    #[sea_orm(string_value = "undelete")]
    Undelete,
}
#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_type")]
pub enum UserType {
    #[sea_orm(string_value = "bot")]
    Bot,
    #[sea_orm(string_value = "regular")]
    Regular,
    #[sea_orm(string_value = "site")]
    Site,
    #[sea_orm(string_value = "system")]
    System,
}
