use collector::db;

#[tokio::main]
async fn main() -> Result<(), db::DatabaseError> {
    let database_config = db::DatabaseConfig::from_env()?;
    let database_pool = db::create_pool(&database_config)?;

    db::run_migrations(&database_pool).await?;

    println!("collector database migrations completed");

    Ok(())
}
