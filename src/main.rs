use clap::Parser;
use cli::Commands;
use sea_orm::DatabaseConnection;
use tracing_subscriber;
use web::init_webserver;

//
// NOTE: Import web for axum routing and all web related
//
pub mod web;

pub mod database;

pub mod utils;

pub mod config;

pub mod cli;

#[tokio::main]
async fn main() {
    //  NOTE: initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    //  NOTE: Connect to db and create app state
    let db = database::get_database_connection().await;

    if !(database::ping_db(db.clone()).await) {
        panic!("No connection to the database could be found");
    }

    let args = cli::Args::parse();

    match args.command {
        Commands::Serve => {
            start_webserver(db).await;
        }
        Commands::Migrate { command } => match command {
            cli::MigrateCommands::Up => {
                database::apply_migrations(&db).await;
            }
            cli::MigrateCommands::Down => {
                database::undo_migrations(&db).await;
            }
        },
    }
}

async fn start_webserver(db: DatabaseConnection) {
    let state = AppState { db };

    // NOTE: start webserver
    init_webserver(state).await;
}

#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
}
