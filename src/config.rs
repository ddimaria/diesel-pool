use crate::database::DatabaseConnection;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_connection: DatabaseConnection,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
          database_connection: DatabaseConnection::Mysql,
          database_url: "mysql://root:root@127.0.0.1:8889/rust-actix-framework?socket=/Applications/MAMP/tmp/mysql/mysql.sock".into()
      }
    }
}
