use clap::Parser;
use cli::Commands;
use sqlx::SqlitePool;
use tracing_subscriber;
use web::init_webserver;

//
// NOTE: Import web for axum routing and all web related
//
pub mod web;

pub mod utils;

pub mod database;

pub mod config;

pub mod cli;

#[tokio::main]
async fn main() {
    //  NOTE: initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let args = cli::Args::parse();

    //  NOTE: Database constructor
    let pool = database::get_database_driver()
        .await
        .expect("Error loading database");

    match args.command {
        Commands::Serve => {
            start_webserver(pool).await;
        }
        Commands::Migrate { command } => match command {
            cli::MigrateCommands::Up => {
                database::migration_up(&pool)
                    .await
                    .expect("Something went wrong running migrations");
            }
            cli::MigrateCommands::Down => {
                database::migration_down(&pool)
                    .await
                    .expect("Something went wrong running migrations");
            }
            cli::MigrateCommands::New => {
                database::new_migration();
            }
        },
    }
}

async fn start_webserver(pool: SqlitePool) {
    let state = AppState { pool };

    // NOTE: start webserver
    init_webserver(state).await;
}

#[derive(Clone)]
pub struct AppState {
    pool: SqlitePool,
}
