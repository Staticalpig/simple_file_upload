use postgres::{Client, NoTls};
use crate::db_handling::{DbConfig};


pub fn connect_to_db(configuration: DbConfig) -> Result<Client, postgres::Error> {
    let client = Client::connect(
        &format!(
            "host={} port={} user={} password={} dbname={}",
            configuration.host,
            configuration.port,
            configuration.user,
            configuration.password,
            configuration.database
        ),
        NoTls,
    )?;

    Ok(client)
}