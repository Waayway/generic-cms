use sea_orm::entity::prelude::*;
use sea_orm_migration::{prelude::*, schema::*};

use crate::{alter_table, create_table, drop_table};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// Migrations

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        create_table!(manager, "user", {
            ("id" => pk_auto),
            ("name" => string_uniq),
            ("email" => string_uniq),
            ("password" => string)
        });
        alter_table!(manager, "user", {
            (add_fk, "FK_role", ("user", "roleId"), ("role", "id"))
        });

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        drop_table!(manager, "user");
        Ok(())
    }
}
