//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "accounts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub hash: Vec<u8>,
    pub data: Vec<u8>,
    pub data_hash: Option<Vec<u8>>,
    pub discriminator: Vec<u8>,
    pub address: Option<Vec<u8>>,
    pub owner: Vec<u8>,
    pub tree: Option<Vec<u8>>,
    pub leaf_index: Option<i64>,
    pub seq: Option<i64>,
    pub slot_updated: i64,
    pub spent: bool,
    pub created_at: Option<DateTime>,
    #[sea_orm(column_type = "Decimal(Some((20, 0)))")]
    pub lamports: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::token_accounts::Entity")]
    TokenAccounts,
}

impl Related<super::token_accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TokenAccounts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
