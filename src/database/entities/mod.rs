use sea_orm_migration::MigrationTrait;

pub mod role;
pub mod user;

pub fn get_migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![Box::new(user::Migration)]
}
