pub mod postgress;

const TIMEOUT_MS: u64 = 5000; // 5000 milliseconds (5 seconds)

pub struct DbConfig  {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}
