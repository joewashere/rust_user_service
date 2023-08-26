#[macro_use]
extern crate rocket;

mod routes;
mod db;
mod auth;
mod config;

use db::conn::{init_database_pool, setup_database};
use routes::user::{register_v1, login_v1, delete_v1};
use crate::config::load_configuration;
use std::fs::OpenOptions;
use std::path::Path;

#[launch]
pub async fn rocket() -> _ {
    let settings = load_configuration().expect("Failed to load configuration.");

    // Explicitly create the database file if it doesn't exist
    let database_path = Path::new("users.db");
    if !database_path.exists() {
        OpenOptions::new().write(true).create(true).open(&database_path)
            .expect("Failed to create database file.");
    }

    // Use an absolute path for the SQLite connection
    let database_url = format!("sqlite://{}", database_path.to_string_lossy());

    // Setup the database
    if let Err(e) = setup_database(&database_url).await {
        eprintln!("Failed to set up the database: {:?}", e);
    }

    // Initialize the database connection pool
    let db_pool = init_database_pool(&database_url).await.expect("Failed to initialize database pool.");

    let rocket_instance = rocket::build()
        .mount("/", routes![register_v1, login_v1, delete_v1])
        .manage(db_pool)
        .manage(settings);

    rocket_instance
}