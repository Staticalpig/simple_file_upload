use postgres::{Client, NoTls};

let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

