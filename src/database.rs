//! Database-related functions
use crate::config::Config;
use crate::database_error::DatabaseError;
use diesel::{mysql::MysqlConnection, pg::PgConnection, sqlite::SqliteConnection, Connection};

#[serde(untagged)]
#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(field_identifier, rename_all = "lowercase")]
pub enum DatabaseConnection {
    Cockroach,
    Mysql,
    Postgres,
    Sqlite,
}

pub enum InferConnection {
    Cockroach(PgConnection),
    Mysql(MysqlConnection),
    Postgres(PgConnection),
    Sqlite(SqliteConnection),
}

impl InferConnection {
    pub fn establish(config: Config) -> Result<Self, DatabaseError> {
        match config.database_connection {
            DatabaseConnection::Cockroach => {
                PgConnection::establish(&config.database_url).map(InferConnection::Postgres)
            }
            DatabaseConnection::Mysql => {
                MysqlConnection::establish(&config.database_url).map(InferConnection::Mysql)
            }
            DatabaseConnection::Postgres => {
                PgConnection::establish(&config.database_url).map(InferConnection::Postgres)
            }
            DatabaseConnection::Sqlite => {
                SqliteConnection::establish(&config.database_url).map(InferConnection::Sqlite)
            }
        }
        .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use crate::database::*;

    #[test]
    fn it_works() {
        let config = Config::new();
        let conn = InferConnection::establish(config);
    }
}
