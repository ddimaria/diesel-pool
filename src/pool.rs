//! Database-related functions
use crate::config::Config;
use crate::database::DatabaseConnection;
use diesel::{
    mysql::MysqlConnection,
    pg::PgConnection,
    r2d2::{ConnectionManager, PoolError},
    sqlite::SqliteConnection,
    Connection,
};

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type CockroachPool = Pool<PgConnection>;
pub type MysqlPool = Pool<MysqlConnection>;
pub type PostgresPool = Pool<PgConnection>;
pub type SqlitePool = Pool<SqliteConnection>;

pub enum InferPool {
    Cockroach(CockroachPool),
    Mysql(MysqlPool),
    Postgres(PostgresPool),
    Sqlite(SqlitePool),
}

impl InferPool {
    pub fn init_pool(config: Config) -> Result<Self, r2d2::Error> {
        match config.database_connection {
            DatabaseConnection::Cockroach => {
                init_pool::<PgConnection>(config).map(InferPool::Cockroach)
            }
            DatabaseConnection::Mysql => init_pool::<MysqlConnection>(config).map(InferPool::Mysql),
            DatabaseConnection::Postgres => {
                init_pool::<PgConnection>(config).map(InferPool::Postgres)
            }
            DatabaseConnection::Sqlite => {
                init_pool::<SqliteConnection>(config).map(InferPool::Sqlite)
            }
        }
        .map_err(Into::into)
    }
}

pub fn init_pool<T>(config: Config) -> Result<Pool<T>, PoolError>
where
    T: Connection + 'static,
{
    let manager = ConnectionManager::<T>::new(config.database_url);
    Pool::builder().build(manager)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let config = Config::new();
        let pool = InferPool::init_pool(config);
    }
}
