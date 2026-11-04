pub mod postgres_handler;

#[derive(Debug)]
pub struct DbConfig  {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl DbConfig {
    pub fn new(host: String, port: u16, user: String, password: String, database: String) -> Self {
        DbConfig {
            host,
            port,
            user,
            password,
            database,
        }
    }

}