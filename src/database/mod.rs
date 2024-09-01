use std::{
    fs,
    io::{self, Write},
};

use chrono::Local;
use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions, SqlitePool};

use crate::config::Config;

pub async fn get_database_driver() -> Result<SqlitePool, sqlx::Error> {
    let config = Config::get_env();
    let pool = SqlitePoolOptions::new().connect(&config.database_url).await;

    pool
}

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn migration_up(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    MIGRATOR.run(pool).await?;
    Ok(())
}
pub async fn migration_down(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    MIGRATOR.undo(pool, 0).await?;
    Ok(())
}

// Used via cli to create new migrations
pub fn new_migration() {
    let mut input = String::new();

    print!("Please enter the name of the migration file: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading from stdin");

    input = input.replace("\n", "");

    let mut file_prefix: String = Local::now().format("%Y%m%d").to_string();

    let migrations: Vec<String> = fs::read_dir("./migrations")
        .unwrap()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.file_name().into_string().ok())
        .collect();

    let amount = migrations
        .iter()
        .filter(|&x| x.starts_with(&file_prefix))
        .count();
    file_prefix = format!("{}-{amount:0>3}", file_prefix);

    let file_name = format!("{file_prefix}-{input}.sql");

    println!("Creating new file... {}", file_name);

    fs::File::create(format!("./migrations/{}", file_name))
        .expect("Couldn't write to the filesystem...");
}
