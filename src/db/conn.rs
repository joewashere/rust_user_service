use sqlx::SqlitePool;

pub struct UsersDbConn(pub SqlitePool);

pub async fn init_database_pool(database_url: &str) -> Result<UsersDbConn, sqlx::Error> {
    let pool = SqlitePool::connect(database_url).await?;
    Ok(UsersDbConn(pool))
}

pub async fn setup_database(database_url: &str) -> sqlx::Result<()> {
    let pool = SqlitePool::connect(database_url).await?;
    let mut conn = pool.acquire().await?;

    sqlx::query("CREATE TABLE IF NOT EXISTS users (username TEXT PRIMARY KEY, password TEXT NOT NULL)")
        .execute(&mut conn)
        .await?;

    Ok(())
}