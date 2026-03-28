use std::{env, time::Duration};

use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn get_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL NOT FOUND");

    let pool = PgPoolOptions::new()
        .min_connections(3)
        .max_connections(15)
        .idle_timeout(Duration::from_secs(60 * 10))
        .max_lifetime(Duration::from_secs(60 * 30))
        .connect_lazy(&database_url)?;

    sqlx::migrate!().run(&pool).await.expect("MIGRARION FAILED");
    Ok(pool)
}
