use std::net::TcpListener;
pub mod api;
pub mod domain;
pub mod persistence;
pub mod view;
use actix_cors::Cors;
use actix_session::SessionMiddleware;
use actix_web::{
    App, HttpServer,
    cookie::{Key, SameSite},
    dev::Server,
    http::header,
    web,
};
use sqlx::PgPool;

use crate::api::{route, session::PgSessionStore};

pub async fn run(
    lst: TcpListener,
    db_pool: PgPool,
    secret_key: String,
) -> Result<Server, std::io::Error> {
    let pool_data = web::Data::new(db_pool.clone());

    let store = PgSessionStore::new(db_pool);

    let signing_key = Key::from(secret_key.as_bytes());
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["POST", "GET"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
            .max_age(3600);
        App::new()
            .app_data(pool_data.clone())
            .wrap(
                SessionMiddleware::builder(store.clone(), signing_key.clone())
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Strict)
                    .build(),
            )
            .wrap(cors)
            .configure(route::routes)
    })
    .listen(lst)?
    .run();

    Ok(server)
}
