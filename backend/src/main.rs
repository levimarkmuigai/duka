use std::net::TcpListener;

use backend::run;

pub mod utils;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let lst = TcpListener::bind("127.0.0.1:8000")?;

    run(lst).await?.await
}
