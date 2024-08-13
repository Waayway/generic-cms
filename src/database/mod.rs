use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

use crate::config::Config;

mod migrations;

pub async fn get_database_connection() -> DatabaseConnection {
    let config = Config::get_env();
    let mut opt = ConnectOptions::new(format!(
        "postgres://{}:{}@{}/{}",
        config.db_user, config.db_pass, config.db_host, config.db_name
    ));

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    let db = Database::connect(opt)
        .await
        .expect("Failed to connect to the database");

    db
}

pub async fn ping_db(db: DatabaseConnection) -> bool {
    db.ping().await.is_ok()
}

pub async fn apply_migrations(db: &DatabaseConnection) {
    migrations::Migrator::up(db, None)
        .await
        .expect("Migration failed")
}

pub async fn undo_migrations(db: &DatabaseConnection) {
    migrations::Migrator::down(db, None)
        .await
        .expect("Migration undo failed")
}
