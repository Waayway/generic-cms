use sea_orm::DatabaseConnection;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};
use web::init_webserver;

//
// NOTE: Import web for axum routing and all web related
//
pub mod web;

pub mod database;

pub mod utils;

pub mod config;

#[tokio::main]
async fn main() {
    //  NOTE: initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    //  NOTE: Connect to db and create app state
    let db = database::get_database_connection().await;

    if !(database::ping_db(db.clone()).await) {
        panic!("No connection to the database could be found");
    }

    let state = AppState { db };

    // NOTE: start webserver
    init_webserver(state).await;
}

#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
}
