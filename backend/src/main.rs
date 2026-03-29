use std::net::TcpListener;

use backend::{persistence::db::get_pool, run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let lst = TcpListener::bind("127.0.0.1:8000")?;

    let secret_key = std::env::var("SESSION_KEY").expect("SESSION KEY NOT FOUND");

    let db_pool = get_pool()
        .await
        .expect("FAILED TO CONNECT TO DATABASE ON STARTUP");

    run(lst, db_pool, secret_key).await?.await
}
