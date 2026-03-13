use std::net::TcpListener;
pub mod api;
use actix_web::{App, HttpServer, dev::Server};

use crate::api::route;

pub async fn run(lst: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().configure(route::routes))
        .listen(lst)?
        .run();
    Ok(server)
}
