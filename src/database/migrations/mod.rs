use axum::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

mod m20240807_00001_create_user_tables;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![]
    }
}
