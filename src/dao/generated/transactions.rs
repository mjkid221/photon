//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "transactions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub signature: Vec<u8>,
    pub slot: i64,
    pub uses_compression: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::blocks::Entity",
        from = "Column::Slot",
        to = "super::blocks::Column::Slot",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Blocks,
}

impl Related<super::blocks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Blocks.def()
    }
}

impl Related<super::accounts::Entity> for Entity {
    fn to() -> RelationDef {
        super::account_transactions::Relation::Accounts.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::account_transactions::Relation::Transactions
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
