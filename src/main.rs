#![deny(unused_must_use, clippy::unwrap_used)]
#![forbid(unsafe_code)]

mod db_handling;
mod logger;

use db_handling::postgres_handler;
use logger::custom_logger::CustomLogger;

static LOGGER: CustomLogger = CustomLogger;

fn get_db_config() -> db_handling::DbConfig {
    let db_host = std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".into());
    let db_port = std::env::var("DB_PORT").unwrap_or_else(|_| "5432".into());
    let db_user = std::env::var("DB_USER").unwrap_or_else(|_| "postgres".into());
    let db_password = std::env::var("DB_PASSWORD").unwrap_or_else(|_| "password".into());
    let db_name = std::env::var("DB_NAME").unwrap_or_else(|_| "file_upload".into());

    log::info!("Database Configuration:
        Host: {}
        Port: {}
        User: {}
        Password: {}
        Database: {}",
        db_host, db_port, db_user, db_password, db_name);

    db_handling::DbConfig::new(
        db_host,
        db_port.parse().unwrap_or(5432),
        db_user,
        db_password,
        db_name,
    )
}

fn main() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Info))
        .expect("Failed to set logger");

    let _client = match postgres_handler::connect_to_db(get_db_config()) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error connecting to the database: {}", e);
            return;
        }
    };



}