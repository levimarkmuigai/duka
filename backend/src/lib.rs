use std::net::TcpListener;
pub mod api;
pub mod domain;
pub mod persistence;
pub mod utils;
use actix_cors::Cors;
use actix_web::{App, HttpServer, dev::Server, http::header};

use crate::api::route;

pub async fn run(lst: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["POST", "GET"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
            .max_age(3600);
        App::new().wrap(cors).configure(route::routes)
    })
    .listen(lst)?
    .run();

    Ok(server)
}
