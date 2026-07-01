use std::{env, error::Error as StdError, num::ParseIntError};

use deadpool_diesel::postgres::{BuildError, Manager, Pool, Runtime};
use diesel::{Connection, PgConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use thiserror::Error;

const DEFAULT_POOL_SIZE: usize = 16;
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type DbPool = Pool;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_size: usize,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, DatabaseError> {
        let database_url =
            env::var("DATABASE_URL").map_err(|_| DatabaseError::MissingDatabaseUrl)?;
        let max_size = match env::var("DATABASE_POOL_MAX_SIZE") {
            Ok(value) => value.parse()?,
            Err(_) => DEFAULT_POOL_SIZE,
        };

        Ok(Self {
            database_url,
            max_size,
        })
    }
}

pub fn create_pool(config: &DatabaseConfig) -> Result<DbPool, DatabaseError> {
    let manager = Manager::new(config.database_url.clone(), Runtime::Tokio1);

    Pool::builder(manager)
        .max_size(config.max_size)
        .build()
        .map_err(DatabaseError::PoolBuild)
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), DatabaseError> {
    let connection = pool.get().await?;

    connection
        .interact(|connection| connection.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await??;

    Ok(())
}

pub async fn transaction<F, T>(pool: &DbPool, operation: F) -> Result<T, DatabaseError>
where
    F: FnOnce(&mut PgConnection) -> Result<T, diesel::result::Error> + Send + 'static,
    T: Send + 'static,
{
    let connection = pool.get().await?;

    connection
        .interact(move |connection| connection.transaction(operation))
        .await?
        .map_err(DatabaseError::Transaction)
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("DATABASE_URL must be set")]
    MissingDatabaseUrl,
    #[error("invalid DATABASE_POOL_MAX_SIZE")]
    InvalidPoolSize(#[from] ParseIntError),
    #[error("failed to build database pool")]
    PoolBuild(#[source] BuildError),
    #[error("failed to get database connection from pool")]
    Pool(#[from] deadpool_diesel::PoolError),
    #[error("database operation panicked")]
    Interaction(#[from] deadpool_diesel::InteractError),
    #[error("failed to run database migrations")]
    Migration(#[from] Box<dyn StdError + Send + Sync>),
    #[error("database transaction failed")]
    Transaction(#[source] diesel::result::Error),
}
