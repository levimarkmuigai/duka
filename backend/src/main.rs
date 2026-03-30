use std::net::TcpListener;

use backend::{persistence::db::get_pool, run, utils::telemetry::init_telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    init_telemetry();

    let lst = TcpListener::bind("127.0.0.1:8000")?;

    let secret_key = std::env::var("SESSION_KEY").expect("SESSION KEY NOT FOUND");

    let db_pool = get_pool()
        .await
        .expect("FAILED TO CONNECT TO DATABASE ON STARTUP");

    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("MIGRATION FAILED");

    tracing::info!("Running database migrations...");

    run(lst, db_pool, secret_key).await?.await
}
