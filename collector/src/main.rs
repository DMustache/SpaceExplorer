mod database;

use database::{DatabaseConfig, DatabaseError, create_pool, run_migrations};

#[tokio::main]
async fn main() -> Result<(), DatabaseError> {
    let database_config = DatabaseConfig::from_env()?;
    let database_pool = create_pool(&database_config)?;

    run_migrations(&database_pool).await?;

    println!("collector database migrations completed");

    Ok(())
}
